use std::sync::Arc;

use crate::proxy::handler::handle_conneection;
use crate::{config::Cli, errors::ProxyError};
use tokio::net::TcpListener;

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
    println!("Server starting at: {:?}", listener.local_addr());
    let client = Arc::new(reqwest::Client::new());
    loop {
        let (socket, _) = listener.accept().await?;
        let config = config.clone();
        let client = client.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_conneection(socket, client, &config.target).await {
                eprint!("{:?}", e)
            }
        });
    }
}
