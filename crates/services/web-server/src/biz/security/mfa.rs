use crate::biz::dto::{AuthnMethodEnum, MfaSession};
use crate::http::vo::mfa::{MfaInfo, MfaMethod, MfaVerificationChallenge};
use crate::http::vo::{AppResult, RequestInfo};
use chrono::Utc;
use lib_core::db::models::ProviderType;
use lib_core::db::services::AccountDbService;
use lib_utils::rand_hex_string;
use redis::{AsyncCommands, SetExpiry, SetOptions};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct MultiFactorAuthService {
    redis_client: Arc<redis::Client>,
    account_db_service: Arc<AccountDbService>,
}

impl MultiFactorAuthService {
    pub fn new(
        redis_client: Arc<redis::Client>,
        account_db_service: Arc<AccountDbService>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            redis_client,
            account_db_service,
        })
    }

    pub async fn generate_challenge(
        &self,
        user_id: i64,
        authn_method: &AuthnMethodEnum,
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
        };
        let mfa_session_json = serde_json::to_string(&mfa_session).expect("Failed to serialize");

        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let options = SetOptions::default().with_expiration(SetExpiry::EX(60 * 3));
        let _: () = (&mut conn)
            .set_options(&mfa_session_id, &mfa_session_json, options)
            .await?;

        let challenge = MfaVerificationChallenge {
            user_id,
            mfa_session_id,
            mfa_infos,
        };
        Ok(challenge)
    }

    pub async fn send_challenge(
        &self,
        session_id: &str,
        chosen_method: MfaMethod,
    ) -> AppResult<()> {
        todo!()
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
                    method: MfaMethod::Sms,
                    detail: lib_utils::mask_phone_number(&id.identifier),
                }),
                Some(ProviderType::Email) => Some(MfaInfo {
                    method: MfaMethod::Email,
                    detail: lib_utils::mask_email(&id.identifier),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MfaSessionClaims {
    pub sub: i64,           // Subject (user_id for pending MFA)
    pub session_id: String, // 用于追踪 MFA 会话的 ID
    pub exp: usize,
    // 其他 MFA 会话特有的声明
}
