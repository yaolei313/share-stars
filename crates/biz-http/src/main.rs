mod vo;

use axum::{
    extract::{Json, Path, Query},
    http::{
        header::{self, COOKIE},
        HeaderMap, HeaderName, HeaderValue, StatusCode,
    },
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use axum_extra::{response::Html, TypedHeader};
use headers::UserAgent;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};
use tower::Service;
use vo::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/login_by_password", post(login_by_password))
        .route("/login_by_sms", post(login_by_sms))
        .route("/profile/me", get(profile_me))
        .route("/profile/:user_id", get(profile));
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("bind address fail");
    tracing::debug!("listening on {}", address);
    axum::serve(listener, app).await.expect("serve fail");
}

async fn root(headers: HeaderMap) -> Result<Html<String>, &'static str> {
    let mut login_user_name: Option<String> = None;
    let cookie = headers
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let cs: Vec<&str> = cookie.split(";").collect();
    for c in cs {
        let kv: Vec<&str> = c.split("=").collect();
        let cookie_name = kv[0];
        let cookie_value = kv[1];
        if cookie_name == "user_name" && !cookie_value.is_empty() {
            login_user_name = Some(String::from(cookie_value));
        }
    }
    if login_user_name.is_none() {
        return Err("not found user_name in cookies");
    }
    let html = format!(
        r#" 
<!DOCTYPE html> <html> 
<head> <meta charset="utf-8" />
<title> 用户中心 </title> 
</head>
<body> <p>你好，<strong>{}</strong>！你已成功登录。[<a href="/logout">退出登录</a>] </body> </html>
"#,
        login_user_name.unwrap()
    );
    Ok(Html(html))
}

///
///
async fn login_by_password(
    headers: HeaderMap,
    Json(payload): Json<LoginByPasswordReq>,
) -> Json<RespVo<LoginResult>> {
    for (key, value) in &headers {
        println!("{} {}", key, value.to_str().ok().unwrap());
    }
    if &payload.phone == "18866668888" && &payload.password == "abc123" {
        let result = LoginResult {
            user_id: 123,
            new_register: false,
            access_token: Some(String::from("123")),
            expire_seconds: 0,
            refresh_token: Some(String::from("refresh token")),
        };
        let vo = RespVo::success(result);
        return Json(vo);
    }
    let vo = RespVo::bad_request(String::from("forbidden"));
    Json(vo)
}

async fn login_by_sms(Json(payload): Json<LoginBySmsReq>) -> Json<LoginResult> {
    todo!()
}

async fn profile_me() -> String {
    format!("user")
}

async fn profile(Path(user_id): Path<u64>, Query(param): Query<HashMap<String, String>>) -> String {
    format!("user id:{}", user_id)
}
