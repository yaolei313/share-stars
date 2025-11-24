use crate::biz::dto::{AuthnMethod, MfaSession};
use crate::biz::verify::{VerifyManager, VerifyScenario};
use crate::http::vo::error::AppError;
use crate::http::vo::mfa::{MfaInfo, MfaMethod, MfaVerificationChallenge};
use crate::http::vo::{AppResult, RequestInfo};
use chrono::Utc;
use lib_core::db::models::ProviderType;
use lib_core::db::services::AccountDbService;
use lib_utils::rand_hex_string;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, SetExpiry, SetOptions};
use std::borrow::Cow;
use std::sync::Arc;

pub struct MultiFactorAuthService {
    redis_client: Arc<redis::Client>,
    account_db_service: Arc<AccountDbService>,
    verify_manager: Arc<VerifyManager>,
}

impl MultiFactorAuthService {
    pub fn new(
        redis_client: Arc<redis::Client>,
        account_db_service: Arc<AccountDbService>,
        verify_manager: Arc<VerifyManager>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            redis_client,
            account_db_service,
            verify_manager,
        })
    }

    pub async fn generate_challenge(
        &self,
        user_id: i64,
        authn_method: AuthnMethod,
        req_info: &RequestInfo,
    ) -> AppResult<MfaVerificationChallenge> {
        let mfa_infos = self.get_available_mfa_infos(user_id).await?;

        let mfa_session_id = format!("mfa:{:x}:{}", user_id, rand_hex_string());
        let mfa_session = MfaSession {
            user_id,
            mfa_infos: mfa_infos.clone(),
            create_time: Utc::now(),
            authn_method: authn_method.clone(),
            req_info: req_info.clone(),
            chosen_method: None,
        };

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        self.save_session(&mut conn, &mfa_session_id, &mfa_session)
            .await?;

        let masked_mfa_infos = mask(mfa_infos);

        let challenge = MfaVerificationChallenge {
            user_id,
            mfa_session_id,
            mfa_infos: masked_mfa_infos,
        };
        Ok(challenge)
    }

    pub async fn send_challenge(
        &self,
        session_id: &str,
        chosen_method: MfaMethod,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        // 1.query and validation
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let session: MfaSession = self
            .get_session_and_validate(&mut conn, session_id, req_info)
            .await?;

        let Some(info) = session
            .mfa_infos
            .iter()
            .find(|item| item.method == chosen_method)
        else {
            return Err(AppError::InvalidArgument(Cow::Borrowed(
                "not supported challenge method",
            )));
        };

        // 2.send
        match info.method {
            MfaMethod::SmsCode => {
                self.verify_manager
                    .send_sms_code(&info.detail, VerifyScenario::Mfa, req_info)
                    .await?
            }
            MfaMethod::EmailCode => {
                self.verify_manager
                    .send_email_code(&info.detail, VerifyScenario::Mfa, req_info)
                    .await?
            }
            _ => Err(AppError::InvalidArgument(Cow::Borrowed(
                "invalid challenge method",
            )))?,
        };

        // 3. update session status
        let updated_session = MfaSession {
            chosen_method: Some(chosen_method),
            ..session
        };
        self.save_session(&mut conn, &session_id, &updated_session)
            .await
    }

    pub async fn verify_challenge(
        &self,
        session_id: &str,
        verify_code: &str,
        req_info: &RequestInfo,
    ) -> AppResult<()> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let session: MfaSession = self
            .get_session_and_validate(&mut conn, session_id, req_info)
            .await?;

        let Some(chosen_method) = session.chosen_method else {
            return Err(AppError::InvalidArgument(Cow::Borrowed("invalid session")));
        };
        let Some(info) = session
            .mfa_infos
            .iter()
            .find(|item| item.method == chosen_method)
        else {
            return Err(AppError::InvalidArgument(Cow::Borrowed("invalid method")));
        };

        match chosen_method {
            MfaMethod::SmsCode => {
                self.verify_manager
                    .verify_sms_code(&info.detail, VerifyScenario::Mfa, verify_code, req_info)
                    .await?
            }
            MfaMethod::EmailCode => {
                self.verify_manager
                    .verify_email_code(&info.detail, VerifyScenario::Mfa, verify_code, req_info)
                    .await?
            }
            _ => {
                // todo,增加Totp支持
                Err(AppError::InvalidArgument(Cow::Borrowed(
                    "invalid challenge method",
                )))?
            }
        };

        // 清理挑战
        let _: () = conn.del(session_id).await?;

        Ok(())
    }

    async fn get_session_and_validate(
        &self,
        conn: &mut MultiplexedConnection,
        session_id: &str,
        req_info: &RequestInfo,
    ) -> AppResult<MfaSession> {
        let val: Option<String> = conn.get(session_id).await?;
        let Some(val) = val else {
            return Err(AppError::InvalidArgument(Cow::Borrowed(
                "invalid session_id",
            )));
        };
        let session: MfaSession = serde_json::from_str(&val)
            .map_err(|_| AppError::InvalidArgument(Cow::Borrowed("invalid session_id")))?;
        if session.req_info.device_id != req_info.device_id {
            return Err(AppError::InvalidArgument(Cow::Borrowed(
                "invalid session_id",
            )));
        }
        Ok(session)
    }

    async fn save_session(
        &self,
        conn: &mut MultiplexedConnection,
        session_id: &str,
        session: &MfaSession,
    ) -> AppResult<()> {
        let new_mfa_session_json = serde_json::to_string(session).map_err(|e| {
            AppError::InternalServerError(Cow::Owned(format!("Failed to serialize session: {}", e)))
        })?;
        let options = SetOptions::default().with_expiration(SetExpiry::EX(60 * 3));
        let _: () = conn
            .set_options(session_id, &new_mfa_session_json, options)
            .await?;
        Ok(())
    }

    async fn get_available_mfa_infos(&self, user_id: i64) -> AppResult<Vec<MfaInfo>> {
        let identities = self
            .account_db_service
            .query_identities(
                user_id,
                vec![ProviderType::PhoneNumber.code(), ProviderType::Email.code()],
            )
            .await?;
        let mut result: Vec<MfaInfo> = identities
            .into_iter()
            .filter_map(|id| match ProviderType::from_code(id.provider) {
                Some(ProviderType::PhoneNumber) => Some(MfaInfo {
                    method: MfaMethod::SmsCode,
                    detail: id.identifier,
                }),
                Some(ProviderType::Email) => Some(MfaInfo {
                    method: MfaMethod::EmailCode,
                    detail: id.identifier,
                }),
                _ => None,
            })
            .collect();
        if self.is_bind_totp(user_id).await? {
            result.push(MfaInfo {
                method: MfaMethod::Totp,
                detail: "".to_string(),
            })
        }
        result.sort_by(|a, b| a.method.cmp(&b.method));
        Ok(result)
    }

    async fn is_bind_totp(&self, user_id: i64) -> AppResult<bool> {
        // todo add db query
        Ok(false)
    }
}

fn mask(info: Vec<MfaInfo>) -> Vec<MfaInfo> {
    info.into_iter()
        .map(|i| match i.method {
            MfaMethod::SmsCode => MfaInfo {
                detail: lib_utils::mask_e164_phone_number(&i.detail),
                ..i
            },
            MfaMethod::EmailCode => MfaInfo {
                detail: lib_utils::mask_email(&i.detail),
                ..i
            },
            _ => i,
        })
        .collect()
}
