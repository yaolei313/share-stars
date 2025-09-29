use crate::biz::authn::LoginService;
use crate::biz::dto::{AuthnMethod, Identity};
use crate::http::vo::login::LoginResult;
use crate::http::vo::sms::SmsType;
use crate::http::vo::{AppResult, RequestInfo};

impl LoginService {
    pub async fn login_by_sms(
        &self,
        e164_phone: &str,
        sms_code: &str,
        req_info: &RequestInfo,
    ) -> AppResult<LoginResult> {
        // 1.校验验证码
        self.sms_service
            .validate_sms_code(e164_phone, SmsType::Login, sms_code)
            .await?;

        let identity = Identity::PhoneNumber(e164_phone);
        let account = self.query_then_check_status(&identity).await?;
        let user_id = match account {
            Some(account) => account.user_id,
            None => self.register(&identity).await?,
        };

        self.do_login(user_id, false, AuthnMethod::PhonePassword, req_info)
            .await
    }
}
