use reqwest::{Response, StatusCode, Version};
use std::io::Write;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{cache::policy::CacheMode, error::ProxyError};

#[derive(Debug, PartialEq, Eq)]
pub struct ProxyResponse {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Vec<u8>,
    pub body: Vec<u8>,
}

impl ProxyResponse {
    pub fn new(res: &Response, body: Vec<u8>, headers: Vec<u8>) -> Self {
        Self {
            version: res.version(),
            status: res.status(),
            headers,
            body,
        }
    }
}

#[tracing::instrument(name = "Responding to user", skip_all)]
pub async fn respond(
    mut stream: TcpStream,
    res: &mut Response,
    cache_mode: CacheMode,
) -> Result<Option<ProxyResponse>, ProxyError> {
    let headers = get_headers(&res)?;
    stream.write_all(&headers).await?;

    let mut body_buf = if cache_mode == CacheMode::Cache {
        Some(Vec::new())
    } else {
        None
    };

    while let Some(chunk) = res.chunk().await? {
        stream.write_all(&chunk).await?;

        if let Some(buf) = body_buf.as_mut() {
            buf.extend_from_slice(&chunk);
        }
    }

    if let Some(body) = body_buf {
        let response = ProxyResponse::new(&res, body, headers);
        return Ok(Some(response));
    }

    Ok(None)
}

pub fn get_body_len(res: &Response) -> Option<usize> {
    res.headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

fn get_headers(res: &Response) -> Result<Vec<u8>, ProxyError> {
    let mut headers: Vec<u8> = Vec::with_capacity(512);
    write!(&mut headers, "{:?} {}\r\n", res.version(), res.status())?;

    res.headers().iter().try_for_each(|(k, v)| {
        write!(
            &mut headers,
            "{}: {}\r\n",
            k.as_str(),
            v.to_str().unwrap_or("")
        )
    })?;

    headers.extend_from_slice(b"\r\n");

    Ok(headers)
}
