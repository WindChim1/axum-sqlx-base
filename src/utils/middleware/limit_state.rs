use std::{net::IpAddr, sync::Arc, time::Duration};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_client_ip::SecureClientIp;
use timedmap::TimedMap;

pub struct RateLimiter {
    pub map: TimedMap<IpAddr, i32>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn check(&self, ip_addr: &IpAddr) -> bool {
        let count = self.map.get(ip_addr).unwrap_or(0);
        if count > 10 {
            false
        } else {
            self.map
                .insert(*ip_addr, count + 1, Duration::from_secs(60));
            true
        }
    }
}

#[derive(Clone)]
pub struct LimitState {
    pub limiter: Arc<RateLimiter>,
}

impl LimitState {
    pub fn new() -> Self {
        Self {
            limiter: Arc::new(RateLimiter::new()),
        }
    }
}

pub async fn rate_limit(
    State(state): State<LimitState>,
    SecureClientIp(ip_addr): SecureClientIp,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    tracing::info!(
        "The serever is receiving a request for the host address:{}",
        ip_addr
    );
    if state.limiter.check(&ip_addr) {
        next.run(request).await
    } else {
        (
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests! Please wait for one minute",
        )
            .into_response()
    }
}
