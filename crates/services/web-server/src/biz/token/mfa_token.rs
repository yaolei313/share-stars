use crate::biz::token::JwtManager;
use crate::http::vo::{AppResult, RequestInfo};
use serde::{Deserialize, Serialize};

pub struct MultiFactorTokenService {
    jwt_manager: JwtManager,
}

impl MultiFactorTokenService {
    pub fn new(jwt_manager: JwtManager) -> Self {
        Self { jwt_manager }
    }

    pub fn create_mfa_token(
        &self,
        user_id: i64,
        mfa_flow_id: &str,
        req_info: &RequestInfo,
    ) -> AppResult<String> {
        todo!()
    }

    pub fn validate_mfa_token(&self, token: &str) -> Option<MfaSessionClaims> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MfaSessionClaims {
    pub sub: i64,            // Subject (user_id for pending MFA)
    pub mfa_flow_id: String, // 用于追踪 MFA 会话的 ID
    pub exp: usize,
    // 其他 MFA 会话特有的声明
}
