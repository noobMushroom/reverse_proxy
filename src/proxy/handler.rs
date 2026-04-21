use std::sync::Arc;

use crate::{
    cache::{
        entry::{CacheEntry, CacheKey},
        policy::{CacheMode, should_cache},
        store::CacheStore,
    },
    config::Cli,
    error::ProxyError,
    proxy::{
        request::HttpRequest,
        response::{respond, serve_cache},
        upstream::send_req,
    },
};
use tokio::net::TcpStream;
use tracing::info;

#[tracing::instrument(name = "Handling connection", skip_all)]
pub async fn handle_conneection(
    mut stream: TcpStream,
    client: Arc<reqwest::Client>,
    cache_store: Arc<CacheStore>,
    config: &Cli,
) -> Result<(), ProxyError> {
    stream.readable().await?;
    let mut buffer = [0; 1024];
    let n = stream.try_read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    let http_request = HttpRequest::try_from(request.as_ref())?;
    let cache_key = CacheKey::new(&http_request);

    if let Some(entry) = cache_store.get(&cache_key) {
        info!(event = "Cache-Hit", path = %http_request.path);
        serve_cache(&mut stream, &entry.response).await?;
        return Ok(());
    }

    let mut res = send_req(&http_request, &client, &config.target).await?;

    let mode = if should_cache(&res, config, &http_request) {
        CacheMode::Cache
    } else {
        CacheMode::NoCache
    };

    match mode {
        CacheMode::Cache => {
            info!(event = "Caching and Streaming", path = %http_request.path);
            if let Some(response) = respond(&mut stream, &mut res, mode).await? {
                let cache_val = CacheEntry::new(response, config.ttl);
                cache_store.insert(cache_key, cache_val, config.cache_size);
            }
        }
        CacheMode::NoCache => {
            info!(event = "Streaming", path = %http_request.path);
            respond(&mut stream, &mut res, mode).await?;
        }
    }

    Ok(())
}
