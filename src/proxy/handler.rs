use std::sync::Arc;

use crate::{
    cache::{entry::CacheKey, policy::{CacheMode, should_cache}, store::CacheStore}, config::Cli, error::ProxyError, proxy::{request::HttpRequest, response::respond, upstream::send_req}
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tracing::instrument(name = "Handling connection", skip_all)]
pub async fn handle_conneection(
    stream: TcpStream,
    client: Arc<reqwest::Client>,
    cache_store : Arc<CacheStore>,
    config: &Cli,
) -> Result<(), ProxyError> {
    stream.readable().await?;
    let mut buffer = [0; 1024];
    let n = stream.try_read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    let http_request = HttpRequest::try_from(request.as_ref())?;
    let cache_key = CacheKey::new(&http_request);

    if let Some(v) = cache_store.store.get(&cache_key) {
    };

    let mut res = send_req(&http_request, &client, &config.target).await?;

    if should_cache(&res, config, &http_request) {
        let _response = respond(stream, &mut res, CacheMode::Cache).await?;
    }else {
        respond(stream, &mut res, CacheMode::NoCache).await?;
    }


    Ok(())
}
