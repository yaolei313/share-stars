use crate::http::vo::{DeviceInfo, PlatformEnum};
use axum::extract::{ConnectInfo, FromRequest, FromRequestParts};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::Response;
use std::net::{IpAddr, SocketAddr};

pub struct ExtractDeviceInfo(pub DeviceInfo);

impl<S> FromRequestParts<S> for ExtractDeviceInfo
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ip = get_ip_from_header(&parts.headers)
            .or_else(|| get_ip_from_connect_info(parts))
            .map(|ip| ip.to_string());

        let user_agent = get_header_value(&parts.headers, "user-agent");
        let request_id = get_header_value(&parts.headers, "x-request-id");
        let device_fp = get_header_value(&parts.headers, "app-device-fp");
        let platform = PlatformEnum::Web; // TODO

        Ok(ExtractDeviceInfo(DeviceInfo {
            platform,
            ip,
            user_agent,
            device_fp,
            request_id,
        }))
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
