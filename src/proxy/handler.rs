use std::sync::Arc;

use crate::{
    cache::{
        entry::{CacheEntry, CacheKey},
        policy::{CacheMode, should_cache},
        store::CacheStore,
    },
    config::Cli,
    error::ProxyError,
    proxy::{request::HttpRequest, response::respond, upstream::send_req},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
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
        info!(name: "Cache-Hit", "Cache-Hit to path {}", http_request.path);
        stream.write_all(&entry.response.headers).await?;
        stream.write_all(&entry.response.body).await?;
        return Ok(());
    }

    let mut res = send_req(&http_request, &client, &config.target).await?;

    if should_cache(&res, config, &http_request) {
        info!(name: "streaming and caching", "Caching and Streaming Path: {}", http_request.path);
        let response = respond(stream, &mut res, CacheMode::Cache).await?.unwrap();
        let cache_val = CacheEntry::new(response, config.ttl);
        cache_store.insert(cache_key, cache_val, config.cache_size);
    } else {
        info!(name: "streaming only", "Streaming Path {}", http_request.path);
        respond(stream, &mut res, CacheMode::NoCache).await?;
    }

    Ok(())
}
