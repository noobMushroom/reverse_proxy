use std::time::Instant;
use crate::proxy::{request::{HttpRequest, Method}, response::ProxyResponse};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct CacheKey {
    pub method: Method
    pub path: String,
}


#[derive(PartialEq, Eq, Debug)]
pub struct CacheEntry {
    response: ProxyResponse,
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


// impl CacheEntry {
//    pub fn new(status: u16, ) 
// }
