use crate::biz::authn::LoginService;
use crate::biz::dto::{AuthnMethod, Identity};
use crate::biz::verify::VerifyScenario;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AccessContext, AppResult};

impl LoginService {
    pub async fn login_by_sms(
        &self,
        e164_phone: &str,
        sms_code: &str,
        req_info: &AccessContext,
    ) -> AppResult<LoginResult> {
        // 1.校验验证码
        self.verify_manager
            .verify_sms_code(e164_phone, VerifyScenario::Login, sms_code, req_info)
            .await?;
        tracing::info!("SMS verification passed for phone: {}", e164_phone);

        // 2.查询信息
        let identity = Identity::PhoneNumber(e164_phone);
        let account = self.query_then_check_status(&identity).await?;
        let user_id = match account {
            Some(account) => account.user_id,
            None => self.register(&identity).await?,
        };
        tracing::info!("Starting login process. {}", identity);
        self.do_login(user_id, false, AuthnMethod::PhonePassword, req_info)
            .await
    }
}
