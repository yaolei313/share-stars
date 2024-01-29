use axum::{
    extract::{Json, Path, Query},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};
use tower::Service;

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

async fn login_by_password(Json(payload): Json<LoginByPasswordReq>) -> Json<ResultVo<LoginResult>> {
    if &payload.phone == "18866668888" && &payload.password == "abc123" {
        let result = LoginResult {
            user_id: 123,
            new_register: false,
            access_token: String::from("123"),
            refresh_token: String::from("refresh token"),
        };
        let vo = ResultVo {
            code: 200,
            message: String::from("success"),
            data: Some(result),
        };
        return Json(vo);
    }
    let vo = ResultVo {
        code: 400,
        message: String::from("forbidden"),
        data: None,
    };
    Json(vo)
}

async fn login_by_sms(Json(payload): Json<LoginBySmsReq>) -> Json<LoginResult> {
    todo!()
}

async fn profile_me(Path(user_id): Path<u64>) {}

async fn profile(Query(param): Query<HashMap<String, String>>) {}

#[derive(Deserialize)]
struct LoginByPasswordReq {
    phone: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginBySmsReq {
    phone: String,
    sms_code: String,
}

#[derive(Serialize)]
struct ResultVo<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

#[derive(Serialize)]
struct LoginResult {
    user_id: u64,
    new_register: bool,
    access_token: String,
    refresh_token: String,
}
