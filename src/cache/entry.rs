use crate::proxy::{
    request::{HttpRequest, Method},
    response::ProxyResponse,
};
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct CacheKey {
    pub method: Method,
    pub path: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CacheEntry {
    pub response: ProxyResponse,
    pub expires_at: Instant,
}

impl CacheKey {
    pub fn new(req: &HttpRequest) -> Self {
        Self {
            method: req.method.clone(),
            path: req.path.to_string(),
        }
    }
}

impl CacheEntry {
    pub fn new(response: ProxyResponse, ttl: Duration) -> Self {
        Self {
            response,
            expires_at: Instant::now() + ttl,
        }
    }
}
