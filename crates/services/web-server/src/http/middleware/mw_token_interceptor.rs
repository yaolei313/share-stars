use crate::config::AppState;
use crate::http::vo::error::AppError;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use chrono::{DateTime, Utc};

pub enum LoginMethodEnum {
    PASSWORD,
    SMS,
}
pub struct CurrentUser {
    pub user_id: u64,
    pub login_at: DateTime<Utc>,
    pub login_method: LoginMethodEnum,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            let auth_header = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|value| value.to_str().ok());

            if auth_header.is_none() {
                return Err(AppError::Unauthorized);
            }
            let result = validate_token(auth_header.unwrap(), state).await;
            if result.is_none() {
                Err(AppError::Unauthorized)
            } else {
                Ok(result.unwrap())
            }
        }
    }
}

pub async fn validate_token(token: &str, state: &AppState) -> Option<CurrentUser> {
    if "123" == token {
        Some(CurrentUser {
            user_id: 123456,
            login_at: Default::default(),
            login_method: LoginMethodEnum::PASSWORD,
        })
    } else {
        None
    }
}
