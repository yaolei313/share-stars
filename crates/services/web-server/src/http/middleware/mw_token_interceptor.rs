use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::{header, StatusCode};
use chrono::{DateTime, Utc};
use crate::config::AppState;

pub enum LoginMethodEnum {
    PASSWORD,
    SMS,
}
pub struct CurrentUser {
    pub user_id: u64,
    pub login_at: DateTime<Utc>,
    pub login_method: LoginMethodEnum,
}


impl<S> FromRequestParts<S> for CurrentUser
{
    type Rejection = StatusCode;

    fn from_request_parts(parts: &mut Parts, state: &S) -> impl Future<Output=Result<Self, Self::Rejection>> + Send {
        async {
            let auth_header = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok());

            if auth_header.is_none() {
                return Err(StatusCode::UNAUTHORIZED)
            }
            let state = AppState::from_ref(state);
            let result = validate_token(auth_header.unwrap(), &state).await;
            if result.is_none() {
                Err(StatusCode::UNAUTHORIZED)
            } else {
                Ok(result.unwrap())
            }

        }
    }
}

pub async fn validate_token(token: &str, state: &AppState) -> Option<CurrentUser> {
    if "123" == token {
        Some(CurrentUser { user_id: 123456, login_at: Default::default(), login_method: LoginMethodEnum::PASSWORD })
    } else {
        None
    }
}