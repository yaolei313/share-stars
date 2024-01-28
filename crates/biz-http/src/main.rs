use axum::{
    extract::{Json, Path, Query},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
        .route("/profile/me", get(profile_me))
        .route("/profile", get(profile));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("bind address fail");
    axum::serve(listener, app).await.expect("serve fail");
}

async fn root() -> &'static str {
    "hello world"
}

async fn login_by_password(Json(payload): Json<LoginByPasswordReq>) -> Json<LoginResult> {
    todo!()
}

async fn login_by_sms(Json(payload): Json<LoginBySmsReq>) -> Json<LoginResult> {
    todo!()
}

async fn profile_me(Path(user_id): Path<u64>) {}

async fn profile(Query(param): Query<HashMap<String, String>>) {}

#[derive(Deserialize)]
struct LoginByPasswordReq {}

#[derive(Deserialize)]
struct LoginBySmsReq {}

#[derive(Deserialize)]
struct LoginResult {}
