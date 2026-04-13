use std::sync::Arc;

use crate::{
    errors::ProxyError,
    proxy::{request::HttpRequest, upstream::send_req},
};
use reqwest::Response;
use tokio::{io::AsyncWriteExt, net::TcpStream};

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

pub async fn respond(mut stream: TcpStream, res: Response) -> Result<(), ProxyError> {
    let mut headers = String::new();
    headers.push_str(&format!("{:?} {}\r\n", res.version(), res.status()));
    res.headers().iter().for_each(|(k, v)| {
        headers.push_str(&format!("{}: {}\r\n", k, v.to_str().unwrap_or("")));
    });
    headers.push_str("\r\n");
    stream.writable().await?;
    stream.write_all(&headers.as_bytes()).await?;
    let body = res.bytes().await?;
    stream.write_all(&body).await?;
    Ok(())
}
