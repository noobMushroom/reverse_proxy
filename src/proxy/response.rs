use bytes::{BufMut, Bytes, BytesMut};
use reqwest::{Response, StatusCode, Version};
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{cache::policy::CacheMode, error::ProxyError};

#[derive(Debug, PartialEq, Clone)]
pub struct ProxyResponse {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Bytes,
    pub body_len: usize,
    pub body: Bytes,
}

impl ProxyResponse {
    pub fn new(res: &Response, body: Bytes, headers: Bytes) -> Self {
        let body_len = get_body_len(res).unwrap();

        Self {
            version: res.version(),
            status: res.status(),
            headers,
            body_len,
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
        Some(BytesMut::new())
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
        let response = ProxyResponse::new(&res, body.freeze(), headers);
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

fn get_headers(res: &Response) -> Result<Bytes, ProxyError> {
    let mut headers = BytesMut::with_capacity(1024);

    let status_line = format!("{:?} {}\r\n", res.version(), res.status());

    headers.put(status_line.as_bytes());

    let header_line = res
        .headers()
        .iter()
        .map(|(k, v)| format!("{}: {}\r\n", k.as_str(), v.to_str().unwrap_or("")))
        .chain(std::iter::once("\r\n".to_string()))
        .collect::<String>();

    headers.put(header_line.as_bytes());

    Ok(headers.freeze())
}
