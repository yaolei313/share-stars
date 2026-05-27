use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterByEmailReq {
    /// 邮箱地址，自动进行邮箱格式校验
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,

    /// 密码，限制长度，确保基本强度
    #[validate(length(min = 8, max = 100, message = "密码长度必须在 8-100 位之间"))]
    pub password: String,

    /// 昵称或用户名（可选）
    #[validate(length(min = 2, max = 30, message = "昵称长度必须在 2-30 位之间"))]
    pub username: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmailRegisterResponseData {
    /// 用户在数据库中的唯一标识（建议使用分布式 ID 如 UUID 或 Snowflake）
    pub user_id: String,

    pub email: String,

    /// 账号当前状态：例如 "PendingVerify" (待验证)
    pub status: String,

    /// 提示用户验证码的过期时间（时间戳，单位：秒）
    pub expires_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResult {}
