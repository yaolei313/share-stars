use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Request},
    http::{header::USER_AGENT, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};

pub struct ExtractUserAgent(pub String);

#[async_trait]
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

pub struct ValidatedBody(Bytes);

#[async_trait]
impl<S> FromRequest<S> for ValidatedBody
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;
        Ok(ValidatedBody(body))
    }
}
