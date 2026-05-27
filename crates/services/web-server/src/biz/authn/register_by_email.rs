use crate::biz::authn::RegisterService;
use crate::http::vo::register::EmailRegisterResponseData;
use crate::http::vo::{AccessContext, AppResult};

impl RegisterService {
    pub async fn register_by_email(
        &self,
        email: &str,
        password: &str,
        req_info: &AccessContext,
    ) -> AppResult<EmailRegisterResponseData> {
        todo!()
    }
}
