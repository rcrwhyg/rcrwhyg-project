//! In-process IP rate limiting for public traffic and auth endpoints.

use std::num::NonZeroU32;
use std::sync::{Arc, OnceLock};

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use governor::{Quota, RateLimiter, clock::DefaultClock, state::keyed::DefaultKeyedStateStore};
use std::net::SocketAddr;

type KeyedLimiter = RateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

#[derive(Clone)]
pub struct RateLimitState {
    /// Broad protection for the whole public site (per IP).
    pub global: Arc<KeyedLimiter>,
    /// Stricter cap for login/setup attempts (per IP).
    pub auth: Arc<KeyedLimiter>,
}

impl RateLimitState {
    pub fn from_env() -> Self {
        let global_per_min = env_u32("RATE_LIMIT_PUBLIC_PER_MIN", 180);
        let auth_per_min = env_u32("RATE_LIMIT_AUTH_PER_MIN", 8);
        Self {
            global: Arc::new(RateLimiter::keyed(quota_per_minute(global_per_min, 40))),
            auth: Arc::new(RateLimiter::keyed(quota_per_minute(auth_per_min, 3))),
        }
    }

    pub fn check_auth_ip(&self, ip: &str) -> Result<(), ()> {
        self.auth.check_key(&ip.to_string()).map_err(|_| ())
    }
}

fn env_u32(key: &str, default: u32) -> u32 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn quota_per_minute(per_min: u32, burst: u32) -> Quota {
    let per_min = NonZeroU32::new(per_min.max(1)).unwrap();
    let burst = NonZeroU32::new(burst.max(1)).unwrap();
    Quota::per_minute(per_min).allow_burst(burst)
}

fn shared_state() -> &'static RateLimitState {
    static STATE: OnceLock<RateLimitState> = OnceLock::new();
    STATE.get_or_init(RateLimitState::from_env)
}

pub fn client_ip(headers: &HeaderMap, peer: Option<SocketAddr>) -> String {
    if let Some(fwd) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok())
        && let Some(first) = fwd.split(',').next()
    {
        let trimmed = first.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Some(real) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let trimmed = real.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    peer.map(|a| a.ip().to_string())
        .unwrap_or_else(|| "unknown".into())
}

pub async fn global_rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_ascii_lowercase();
    if path == "/health" {
        return next.run(req).await;
    }

    let state = shared_state();
    let ip = client_ip(req.headers(), Some(addr));
    if state.global.check_key(&ip).is_err() {
        return (StatusCode::TOO_MANY_REQUESTS, "请求过于频繁，请稍后再试。").into_response();
    }

    let is_auth_hot = path.contains("admin_login") || path.ends_with("/admin/login");
    if is_auth_hot && state.auth.check_key(&ip).is_err() {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            "登录尝试过于频繁，请稍后再试。",
        )
            .into_response();
    }

    next.run(req).await
}

pub fn shared_auth_limiter() -> &'static RateLimitState {
    shared_state()
}
