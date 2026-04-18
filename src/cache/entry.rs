use std::time::Instant;
use crate::proxy::{request::Method, response::ProxyResponse};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct CacheKey {
    pub method: Method,
    pub path: String,
}


#[derive(PartialEq, Eq, Debug)]
pub struct CacheEntry {
    response: ProxyResponse,
    pub expires_at: Instant,
}


// impl CacheEntry {
//    pub fn new(status: u16, ) 
// }
