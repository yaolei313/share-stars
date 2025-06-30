use crate::biz::dto::AuthnMethodEnum;
use crate::biz::session;
use crate::config::AppState;
use crate::http::vo::error::AppError;
use crate::http::vo::AppResult;
use axum::extract::{FromRequestParts, OptionalFromRequestParts, Request, State};
use axum::http::header;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: i64,
    pub login_at: DateTime<Utc>,
    pub authn_method: Option<AuthnMethodEnum>,
}

// 使用middle_ware::from_fn + Extension
pub async fn auth_middleware(
    State(state): State<AppState>, // 如果中间件需要访问 AppState
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let Some(token) = auth_header else {
        return Ok(next.run(request).await);
    };

    let Some(current_user) = validate_token(&state, token) else {
        return Ok(next.run(request).await);
    };
    request.extensions_mut().insert(current_user);
    Ok(next.run(request).await)
}
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(current_user) = parts.extensions.get::<CurrentUser>() else {
            return Err(AppError::Unauthorized);
        };
        Ok(current_user.to_owned())
    }
}

impl<S> OptionalFromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts.extensions.get::<CurrentUser>().cloned())
    }
}

// 直接使用 Extractor (不推荐用于认证主流程)
// impl FromRequestParts<AppState> for CurrentUser {
//     type Rejection = AppError;
//
//     fn from_request_parts(
//         parts: &mut Parts,
//         state: &AppState,
//     ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
//         async {
//             let auth_header = parts
//                 .headers
//                 .get(header::AUTHORIZATION)
//                 .and_then(|value| value.to_str().ok());
//
//             if auth_header.is_none() {
//                 return Err(AppError::Unauthorized);
//             }
//             let result = validate_token(state, auth_header.unwrap()).await;
//             if result.is_none() {
//                 Err(AppError::Unauthorized)
//             } else {
//                 Ok(result.unwrap())
//             }
//         }
//     }
// }

pub fn validate_token(state: &AppState, token: &str) -> Option<CurrentUser> {
    let Some(claim) = session::validate_token(state, token) else {
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
