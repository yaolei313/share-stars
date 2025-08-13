use axum::http::{header, HeaderMap};

const BEARER_PREFIX: &str = "Bearer ";

const MFA_PREFIX: &str = "Mfa ";

pub enum TokenSchema {
    Bearer,
    Mfa,
}

impl TokenSchema {
    pub fn extract_token<'a>(&self, headers: &'a HeaderMap) -> Option<&'a str> {
        headers.get(header::AUTHORIZATION).and_then(|value| {
            if let Some(auth_header_value) = value.to_str().ok() {
                if auth_header_value.starts_with(self.prefix()) {
                    return Some(&auth_header_value[self.prefix().len()..]);
                }
            }
            return None;
        })
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            TokenSchema::Bearer => BEARER_PREFIX,
            TokenSchema::Mfa => MFA_PREFIX,
        }
    }
}
