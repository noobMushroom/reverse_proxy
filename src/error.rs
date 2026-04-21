use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Failed to bind Port {port}")]
    Port {
        port: usize,
        #[source]
        source: std::io::Error,
    },

    #[error("I/O Error")]
    I0(#[from] std::io::Error),

    #[error("Invalid Request {0}")]
    InvalidRequest(String),

    #[error("Http Error: {0}")]
    Http(#[from] HttpError),

    #[error("Upstream Error: {0}")]
    UpstreamError(#[from] reqwest::Error),

    #[error("Cache overflow: tried to store {size} bytes, limit is {max}")]
    CacheOverflow { size: usize, max: usize },
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Bad request: {0}")]
    BadRequest(String),
}
