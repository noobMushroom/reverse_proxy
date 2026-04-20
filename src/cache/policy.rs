use crate::{config::Cli, proxy::{request::{HttpRequest, Method}, response::get_body_len}};
use reqwest::Response;

pub fn should_cache(res: &Response, config: &Cli, req: &HttpRequest) -> bool {
    if let Some(len) = get_body_len(res) {
        return len  <= config.max_object_size && req.method == Method::GET; 
    }

    false
}

#[derive(PartialEq, Eq)]
pub enum CacheMode{
    NoCache,
    Cache
}
