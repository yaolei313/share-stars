use crate::http::mw::mw_base::TokenSchema;
use crate::http::vo::error::AppError;
use crate::http::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct CurrentMfaUser {
    pub user_id: i64,
    pub mfa_flow_id: String,
}

pub struct MfaTokenAuth(pub CurrentMfaUser);

impl FromRequestParts<AppState> for MfaTokenAuth {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Some(token) = TokenSchema::Mfa.extract_token(&parts.headers) else {
                return Err(AppError::InvalidMfaToken);
            };
            let Some(current_user) = validate_mfa_token(&state, token) else {
                return Err(AppError::InvalidMfaToken);
            };
            Ok(MfaTokenAuth(current_user))
        }
    }
}

fn validate_mfa_token(state: &AppState, token: &str) -> Option<CurrentMfaUser> {
    let Some(claim) = state.service_state.mfa_service.validate_mfa_token(token) else {
        return None;
    };
    Some(CurrentMfaUser {
        user_id: claim.sub,
        mfa_flow_id: claim.mfa_flow_id,
    })
}
