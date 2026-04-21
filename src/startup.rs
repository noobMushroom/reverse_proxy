use std::sync::Arc;

use crate::cache::store::CacheStore;
use crate::proxy::handler::handle_conneection;
use crate::{config::Cli, error::ProxyError};
use tokio::net::TcpListener;
use tracing::{Instrument, info, info_span};

pub async fn start_listener(port: usize) -> Result<TcpListener, ProxyError> {
    let listener = TcpListener::bind(format!("[::]:{}", port))
        .await
        .map_err(|e| ProxyError::Port {
            port: port,
            source: e,
        })?;

    Ok(listener)
}

pub async fn run(config: Arc<Cli>) -> Result<(), ProxyError> {
    let listener = start_listener(config.port).await?;
    info!("Server starting at {:?}, {}", listener.local_addr(), config.target);
    let client = Arc::new(reqwest::Client::new());
    let store = Arc::new(CacheStore::new());
    loop {
        let (socket, addr) = listener.accept().await?;
        let config = config.clone();
        let client = client.clone();
        let store = store.clone();
        let span = info_span!("request", client_ip=%addr);
        tokio::spawn(
            async move {
                if let Err(e) = handle_conneection(socket, client,store, &config).await {
                    tracing::error!(error = %e, "request failed");
                }
            }
            .instrument(span),
        );
    }
}
