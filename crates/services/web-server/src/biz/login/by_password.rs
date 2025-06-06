use crate::config::AppState;
use crate::http::vo::AppResult;
use crate::http::vo::error::AppError;
use crate::http::vo::login::LoginResult;

pub fn login_by_password(state: AppState, phone: &str, password: &str) -> AppResult<LoginResult> {
    if phone == "18866668888" && password == "abc123" {
        let result = LoginResult {
            user_id: 123,
            new_register: false,
            access_token: Some(String::from("123")),
            expire_seconds: 0,
            refresh_token: Some(String::from("refresh token")),
        };
        return Ok(result);
    }
    Err(AppError::InvalidUserOrPassword)
}
