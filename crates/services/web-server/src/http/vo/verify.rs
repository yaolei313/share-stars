use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyCodePayload {
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 6位数字验证码
    #[validate(length(equal = 6, message = "验证码必须为 6 位"))]
    pub code: String,
}
