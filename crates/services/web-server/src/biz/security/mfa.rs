use crate::biz::token::{JwtManager, MfaSessionClaims, MultiFactorTokenService};
use crate::http::vo::mfa::MfaVerificationChallenge;
use crate::http::vo::AppResult;

pub struct MultiFactorAuthService {
    token_service: MultiFactorTokenService,
}

impl MultiFactorAuthService {
    pub fn new(jwt_manager: JwtManager) -> Self {
        let token_service = MultiFactorTokenService::new(jwt_manager);
        Self { token_service }
    }

    pub async fn generate_challenge(&self) -> AppResult<MfaVerificationChallenge> {
        todo!()
    }

    pub fn validate_mfa_token(&self, token: &str) -> Option<MfaSessionClaims> {
        todo!()
    }
}
