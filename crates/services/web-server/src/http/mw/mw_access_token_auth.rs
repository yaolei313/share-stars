use crate::biz::dto::AuthnMethodEnum;
use crate::http::AppState;
use crate::http::mw::mw_base;
use crate::http::vo::error::AppError;
use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use axum::http::request::Parts;
use chrono::{DateTime, TimeZone, Utc};
use mw_base::TokenSchema;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: i64,
    pub login_at: DateTime<Utc>,
    pub authn_method: Option<AuthnMethodEnum>,
}

pub struct AccessTokenAuth(pub CurrentUser);

pub struct OptionAccessTokenAuth(pub Option<CurrentUser>);

// 使用middle_ware::from_fn + Extension
// pub async fn auth_middleware(
//     State(state): State<AppState>, // 如果中间件需要访问 AppState
//     mut request: Request,
//     next: Next,
// ) -> AppResult<Response> {
//     let auth_header = request
//         .headers()
//         .get(header::AUTHORIZATION)
//         .and_then(|value| value.to_str().ok());
//
//     let Some(token) = auth_header else {
//         return Ok(next.run(request).await);
//     };
//
//     let Some(current_user) = validate_token(&state, token) else {
//         return Ok(next.run(request).await);
//     };
//     request.extensions_mut().insert(current_user);
//     Ok(next.run(request).await)
// }
// impl<S> FromRequestParts<S> for CurrentUser
// where
//     S: Send + Sync,
// {
//     type Rejection = AppError;
//
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         let Some(current_user) = parts.extensions.get::<CurrentUser>() else {
//             return Err(AppError::Unauthorized);
//         };
//         Ok(current_user.to_owned())
//     }
// }
//
// impl<S> OptionalFromRequestParts<S> for CurrentUser
// where
//     S: Send + Sync,
// {
//     type Rejection = AppError;
//
//     async fn from_request_parts(
//         parts: &mut Parts,
//         _state: &S,
//     ) -> Result<Option<Self>, Self::Rejection> {
//         Ok(parts.extensions.get::<CurrentUser>().cloned())
//     }
// }

// 直接使用 Extractor
impl FromRequestParts<AppState> for AccessTokenAuth {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Some(token) = TokenSchema::Bearer.extract_token(&parts.headers) else {
                return Err(AppError::Unauthorized);
            };
            let Some(current_user) = validate_access_token(&state, token) else {
                return Err(AppError::Unauthorized);
            };
            Ok(AccessTokenAuth(current_user))
        }
    }
}

impl FromRequestParts<AppState> for OptionAccessTokenAuth {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            if let Ok(AccessTokenAuth(current_user)) =
                AccessTokenAuth::from_request_parts(parts, state).await
            {
                Ok(OptionAccessTokenAuth(Some(current_user)))
            } else {
                Ok(OptionAccessTokenAuth(None))
            }
        }
    }
}

fn validate_access_token(state: &AppState, token: &str) -> Option<CurrentUser> {
    let Some(claim) = state
        .service_state
        .token_service
        .validate_access_token(token)
    else {
        return None;
    };
    let login_at: DateTime<Utc> = Utc
        .timestamp_opt(claim.iat, 0)
        .single()
        .expect("invalid timestamp");
    Some(CurrentUser {
        user_id: claim.sub,
        login_at,
        authn_method: AuthnMethodEnum::from_code(claim.aum),
    })
}
