use crate::error::{HttpError, ProxyError};
use core::fmt;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub version: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Method {
    GET,
    POST,
    UNKNOWN(String),
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            v if v.eq_ignore_ascii_case("GET") => Method::GET,
            v if v.eq_ignore_ascii_case("POST") => Method::POST,
            unknown => Method::UNKNOWN(unknown.to_string()),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::UNKNOWN(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<&Method> for reqwest::Method {
    type Error = ProxyError;
    fn try_from(value: &Method) -> Result<Self, Self::Error> {
        match value {
            Method::GET => Ok(reqwest::Method::GET),
            Method::POST => Ok(reqwest::Method::POST),
            Method::UNKNOWN(e) => Err(ProxyError::InvalidRequest(format!("Invalid Method: {}", e))),
        }
    }
}

impl TryFrom<&str> for HttpRequest {
    type Error = HttpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let header = value
            .lines()
            .next()
            .ok_or_else(|| HttpError::BadRequest("missing header".into()))?;

        let mut parts = header.split_ascii_whitespace();

        let (method, path, version) = match (parts.next(), parts.next(), parts.next()) {
            (Some(m), Some(p), Some(v)) => (m, p, v),
            _ => return Err(HttpError::BadRequest("Malformed request".into())),
        };

        let method = Method::from(method);

        if matches!(method, Method::UNKNOWN(_)) {
            return Err(HttpError::BadRequest("Unsupported Method".into()));
        }

        if !version.starts_with("HTTP") {
            return Err(HttpError::BadRequest("Invalid Version".into()));
        }

        Ok(Self {
            method,
            path: path.to_string(),
            version: version.to_string(),
        })
    }
}
