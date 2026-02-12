use crate::biz::authn::LoginService;
use crate::biz::dto::{AuthnMethod, Identity};
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;
use crate::http::vo::{AccessContext, AppResult};

impl LoginService {
    pub async fn login_by_password(
        &self,
        e164_phone: &str,
        password: &str,
        req_info: &AccessContext,
    ) -> AppResult<LoginResult> {
        tracing::info!("login by. {}", e164_phone);
        let identity = Identity::PhoneNumber(e164_phone);
        let account = self.query_then_check_status(&identity).await?;
        let Some(account) = account else {
            return Err(AppError::UnregisterAccount);
        };
        self.check_password(password, &account).await?;
        self.do_login(account.user_id, false, AuthnMethod::PhonePassword, req_info)
            .await
    }
}
