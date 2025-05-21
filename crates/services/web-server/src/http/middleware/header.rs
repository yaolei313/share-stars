use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::header::USER_AGENT;
use axum::http::request::Parts;

pub struct ExtractUserAgent(pub String);

impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    #[doc = r#" If the extractor fails it'll use this "rejection" type. A rejection is"#]
    #[doc = r" a kind of error that can be converted into a response."]
    type Rejection = (StatusCode, &'static str);

    #[doc = r" Perform the extraction."]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            let t: &str = user_agent.to_str().unwrap_or_default();
            Ok(ExtractUserAgent(String::from(t)))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}
