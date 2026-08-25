//! HTTP helpers for admin auth server functions (ssr).

use http::header::{COOKIE, HeaderMap, HeaderValue, SET_COOKIE, USER_AGENT};
use http::request::Parts;
use leptos::prelude::*;
use leptos_axum::ResponseOptions;
use sqlx::PgPool;

use super::session::SESSION_COOKIE;

#[derive(Clone, Debug)]
pub struct ClientMeta {
    pub ip: String,
    pub ua: Option<String>,
}

pub fn require_pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("数据库未连接：请配置 DATABASE_URL 并执行 sql/auth.sql"))
}

pub async fn client_meta() -> ClientMeta {
    let headers = request_headers().await;
    let ip = super::rate_limit::client_ip(&headers, None);
    let ua = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    ClientMeta { ip, ua }
}

pub async fn read_session_token() -> Option<String> {
    let headers = request_headers().await;
    token_from_cookie_header(&headers)
}

pub fn token_from_cookie_header(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get(COOKIE)?.to_str().ok()?;
    for part in raw.split(';') {
        let part = part.trim();
        let Some((name, value)) = part.split_once('=') else {
            continue;
        };
        if name.trim() == SESSION_COOKIE {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

pub fn append_set_cookie(value: HeaderValue) -> Result<(), ServerFnError> {
    let response = expect_context::<ResponseOptions>();
    response.append_header(SET_COOKIE, value);
    Ok(())
}

async fn request_headers() -> HeaderMap {
    match leptos_axum::extract::<Parts>().await {
        Ok(parts) => parts.headers,
        Err(_) => HeaderMap::new(),
    }
}
