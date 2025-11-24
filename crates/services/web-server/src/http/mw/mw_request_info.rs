use crate::http::vo::error::AppError;
use crate::http::vo::{NewDeviceRequestInfo, PlatformEnum, RequestInfo};
use axum::extract::{ConnectInfo, FromRequestParts, OptionalFromRequestParts};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue};
use std::borrow::Cow;
use std::net::{IpAddr, SocketAddr};

pub struct ExtractRequestInfo(pub RequestInfo);

impl<S> FromRequestParts<S> for ExtractRequestInfo
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ip = get_ip_from_header(&parts.headers).or_else(|| get_ip_from_connect_info(parts));

        let request_id = get_header_value(&parts.headers, "x-request-id");
        let Some(device_id) = get_header_value(&parts.headers, "x-device-id") else {
            return Err(AppError::InvalidRequest(Cow::Borrowed(
                "not found x-device-id",
            )));
        };

        let platform = PlatformEnum::Web; // TODO

        Ok(ExtractRequestInfo(RequestInfo {
            platform,
            device_id,
            ip,
            request_id,
        }))
    }
}

pub struct ExtractNewDeviceRequestInfo(pub Option<NewDeviceRequestInfo>);

impl<S> OptionalFromRequestParts<S> for ExtractNewDeviceRequestInfo
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        todo!()
    }
}

fn get_header_value(headers: &HeaderMap<HeaderValue>, header_name: &str) -> Option<String> {
    headers
        .get(header_name)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
}

fn get_ip_from_header(headers: &HeaderMap<HeaderValue>) -> Option<IpAddr> {
    client_ip::cf_connecting_ip(headers)
        .ok()
        .or_else(|| client_ip::cloudfront_viewer_address(headers).ok())
        .or_else(|| client_ip::rightmost_x_forwarded_for(headers).ok())
        .or_else(|| client_ip::x_real_ip(headers).ok())
}

fn get_ip_from_connect_info(parts: &mut Parts) -> Option<IpAddr> {
    parts
        .extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
}
