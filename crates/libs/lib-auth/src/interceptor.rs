use axum::{
    extract::Request,
    http::{
        header::{self}
        , StatusCode,
    },
    middleware::Next,
    response::Response
    ,
};
use axum_extra::response::Html;
use std::borrow::Borrow;
use tower::Service;

#[derive(Clone)]
struct CurrentUser {
    user_id: u64,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|item| item.to_str().ok());

    let token = if let Some(token) = auth_header {
        token
    } else {
        println!("not found token in header");
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(user) = authorize_current_user(token).await {
        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser { user_id: 123456 })
}
