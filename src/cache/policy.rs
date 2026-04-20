use crate::{config::Cli, proxy::{request::{HttpRequest, Method}, response::get_body_len}};
use reqwest::Response;

pub fn should_cache(res: &Response, config: &Cli, req: &HttpRequest) -> bool {
    let max_object_size = config.max_object_size as f64 / (1024.0 * 1024.0);

    if let Some(len) = get_body_len(res) {
        return len as f64 <= max_object_size && req.method == Method::GET; 
    }

    false
}

#[derive(PartialEq, Eq)]
pub enum CacheMode{
    NoCache,
    Cache
}
