use crate::http::mw::get_ua_extractor;
use crate::http::vo::error::AppError;
use crate::http::vo::{
    AccessContext, AccessSource, AppMetadata, AppResult, DesktopMetadata, PlatformEnum,
    SystemEnvironment, WebMetadata,
};
use accept_language::intersection;
use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::header::{ACCEPT_LANGUAGE, COOKIE, USER_AGENT};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue};
use cookie::Cookie;
use serde::Deserialize;
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};
use ua_parser::os;
use ua_parser::user_agent;
use ua_parser::{device, Extractor};

pub struct ExtractAccessContext(pub AccessContext);

impl<S> FromRequestParts<S> for ExtractAccessContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let request_id = get_header_value(headers, "x-request-id").map(|id| id.to_string());
        let client_ip = get_ip_from_header(headers).or_else(|| get_ip_from_connect_info(parts));

        // 检查是否存在 X_DEVICE_INFO Header，进行场景分流
        let x_device_info_raw = get_header_value(headers, X_APP_INFO);

        // 1. 尝试解析大厂风格的打包 Header: `xy-info`

        // 2. Web 场景兜底：从 Cookie 和 User-Agent 解析

        if let Some(raw_info) = x_device_info_raw {
            // app或pc客户端
            let raw = serde_urlencoded::from_str::<RawPackedInfo>(raw_info).map_err(|_e| {
                AppError::InvalidRequest(Cow::Borrowed("Missing or invalid 'plat' in x-app-info"))
            })?;
            let context = AccessContext {
                request_id,
                client_ip,
                device_id: raw.did,
                locale: raw.lc.unwrap_or_else(|| DEFAULT_LANGUAGE.into()),
                env: SystemEnvironment {
                    os_family: raw.os,
                    os_version: raw.ov,
                    device_brand: raw.db,
                    device_model: raw.dm,
                },
                source: match raw.st.as_deref() {
                    Some("ios") => AccessSource::Ios(AppMetadata {
                        bundle_id: raw.bid.unwrap_or_default(),
                        app_version: raw.v.unwrap_or_default(),
                        build_number: raw.bn,
                        channel: raw.ch,
                    }),
                    Some("android") => AccessSource::Android(AppMetadata {
                        bundle_id: raw.bid.unwrap_or_default(),
                        app_version: raw.v.unwrap_or_default(),
                        build_number: raw.bn,
                        channel: raw.ch,
                    }),
                    _ => AccessSource::Pc(DesktopMetadata {
                        app_name: raw.bid.unwrap_or_default(),
                        app_version: raw.v.unwrap_or_default(),
                    }),
                },
            };

            return Ok(ExtractAccessContext(context));
        }

        // let device_id = get_cookie(parts, "device_id")
        //     .or_else(|| get_header(parts, "x-device-id"))
        //     .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing device identity".into()))?;
        //
        // Ok(Self {
        //     request_id: get_header(parts, "x-request-id"),
        //     client_ip: None,
        //     device_id,
        //     locale: get_header(parts, "accept-language").unwrap_or_else(|| "zh-CN".into()),
        //     env: SystemEnvironment::default(), // Web 端 env 建议通过解析 UA 填充
        //     source: AccessSource::Web(WebMeta {
        //         user_agent: get_header(parts, "user-agent").unwrap_or_default(),
        //         browser_family: "Browser".into(),
        //         referrer: get_header(parts, "referer"),
        //     }),
        // })

        // web客户端 (不存在 x-device-info)
        let Some(ua_extractor) = get_ua_extractor() else {
            tracing::warn!("user agent extractor unavailable");
            return Err(AppError::InternalServerError(Cow::Borrowed(
                "not found ua extractor",
            )));
        };

        let device_id = get_header_value(&parts.headers, COOKIE.as_str())
            .and_then(|cookie_str| {
                // 使用 cookie::Cookie::parse_header 来解析，比手动 split 更安全
                // 遍历所有可能的 Cookie
                cookie_str.split(';').find_map(|s| {
                    let cookie_result = Cookie::parse(s.trim());
                    if let Ok(cookie) = cookie_result {
                        if cookie.name() == COOKIE_DEVICE_ID_KEY {
                            return Some(cookie.value().to_string());
                        }
                    }
                    None
                })
            })
            .ok_or(AppError::InvalidRequest(Cow::Borrowed(
                "Missing or invalid 'device_id' in cookie",
            )))?;
        // 提取 Accept-Language (使用 get_header_value)
        let locale = get_header_value(&parts.headers, ACCEPT_LANGUAGE.as_str())
            .map(|raw| {
                let langs = intersection(raw, &SUPPORTED_LANGUAGES);
                let mut langs_iter = langs.into_iter();
                langs_iter.next()
            })
            .flatten()
            .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string());

        let (env, metadata) = extract_web_info(ua_extractor, &parts.headers)?;
        let source = AccessSource::Web(metadata);

        Ok(ExtractAccessContext(AccessContext {
            request_id,
            client_ip,
            device_id,
            locale,
            env,
            source,
        }))
    }
}

fn get_header_value<'a>(
    headers: &'a HeaderMap<HeaderValue>,
    header_name: &'static str,
) -> Option<&'a str> {
    headers.get(header_name).and_then(|h| h.to_str().ok())
}

fn get_ip_from_header(headers: &HeaderMap<HeaderValue>) -> Option<IpAddr> {
    client_ip::cf_connecting_ip(headers)
        .ok()
        .or_else(|| client_ip::cloudfront_viewer_address(headers).ok())
        .or_else(|| client_ip::rightmost_x_forwarded_for(headers).ok())
        .or_else(|| client_ip::x_real_ip(headers).ok())
}

fn get_ip_from_connect_info(parts: &Parts) -> Option<IpAddr> {
    parts
        .extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}

fn extract_web_info(
    extractor: &Extractor<'static>,
    headers: &HeaderMap<HeaderValue>,
) -> AppResult<(SystemEnvironment, WebMetadata)> {
    let user_agent = get_header_value(headers, USER_AGENT.as_str()).ok_or_else(|| {
        tracing::warn!("user_agent is none");
        AppError::InvalidRequest(Cow::Borrowed("Missing or invalid 'userAgent'"))
    })?;

    let (ua, os, device) = extractor.extract(&user_agent);

    let (browser_family, browser_version) =
        if let Some(user_agent::ValueRef { family, major, .. }) = ua {
            (Some(family.to_string()), major.map(|t| t.to_string()))
        } else {
            (None, None)
        };
    let (os_family, os_version) = if let Some(os::ValueRef { os, major, .. }) = os {
        (Some(os.to_string()), major.map(|m| m.into_owned()))
    } else {
        (None, None)
    };
    let (device_brand, device_model) = if let Some(device::ValueRef {
        device: _device,
        brand,
        model,
    }) = device
    {
        (brand.map(|s| s.into_owned()), model.map(|m| m.into_owned()))
    } else {
        (None, None)
    };
    let env = SystemEnvironment {
        os_family,
        os_version,
        device_brand,
        device_model,
    };
    let metadata = WebMetadata {
        user_agent: user_agent.into(),
        browser_family,
        browser_version,
        referrer: None,
    };
    Ok((env, metadata))
}

const X_APP_INFO: &str = "x-app-info";
const COOKIE_DEVICE_ID_KEY: &str = "device_id";
const SUPPORTED_LANGUAGES: [&'static str; 2] = ["zh-CN", "en-US"];
const DEFAULT_LANGUAGE: &str = "zh-CN";

#[derive(Deserialize)]
struct RawPackedInfo {
    did: String,         // device_id
    bid: Option<String>, // bundle_id
    v: Option<String>,   // version
    os: Option<String>,  // os_family
    ov: Option<String>,  // os_version
    db: Option<String>,  // device_brand (设备品牌，如 Apple, Huawei, Xiaomi)
    dm: Option<String>,  // device_model
    st: Option<String>,  // source_type: ios, android, pc
    lc: Option<String>,  // locale (如 zh-CN)
    bn: Option<String>,  // build_number (构建版本号，如 "1024")
    ch: Option<String>,  // channel (渠道号，如 "AppStore", "GooglePlay")
}
