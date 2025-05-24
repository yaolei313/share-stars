use axum::http::HeaderMap;
use axum::http::header::COOKIE;
use axum::response::Html;
use axum_extra::TypedHeader;
use headers::UserAgent;

pub async fn root(
    headers: HeaderMap,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<Html<String>, &'static str> {
    println!("user agent: {}", user_agent);
    let mut login_user_name: Option<String> = None;
    let cookie = headers
        .get(COOKIE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let cs: Vec<&str> = cookie.split(";").collect();
    for c in cs {
        let kv: Vec<&str> = c.split("=").collect();
        if kv.len() != 2 {
            continue;
        }
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
