use std::sync::Arc;

use crate::{
    error::ProxyError,
    proxy::{request::HttpRequest, response::ProxyResponse, upstream::send_req},
};
use reqwest::Response;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::info;

#[tracing::instrument(name = "Handling connection", skip_all)]
pub async fn handle_conneection(
    stream: TcpStream,
    client: Arc<reqwest::Client>,
    target: &str,
) -> Result<(), ProxyError> {
    stream.readable().await?;
    let mut buffer = [0; 1024];
    let n = stream.try_read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    let http_request = HttpRequest::try_from(request.as_ref())?;
    let res = send_req(&http_request, &client, &target).await?;
    respond(stream, res).await?;
    Ok(())
}

#[tracing::instrument(name = "Responding to user", skip_all)]
pub async fn respond(mut stream: TcpStream, res: Response) -> Result<(), ProxyError> {
    stream.writable().await?;
    let res = ProxyResponse::from_reqwest(res).await?;
    stream.write_all(&res.get_status_line().as_bytes()).await?;
    stream.write_all(&res.get_headers().as_bytes()).await?;
    stream.write_all(&res.body).await?;
    info!("Responded to user written {} bytes", res.body.len());
    Ok(())
}
