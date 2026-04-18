use reqwest::{Response, StatusCode, Version};

use crate::error::ProxyError;

#[derive(Debug, PartialEq, Eq)]
pub struct ProxyResponse {
    pub version: Version,
    pub status: StatusCode,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl ProxyResponse {
    pub async fn from_reqwest(response: Response) -> Result<Self, ProxyError> {
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect::<Vec<(String, String)>>();
        let version = response.version();
        let status = response.status();

        let body = response.bytes().await?;

        Ok(Self {
            version,
            status,
            headers,
            body: body.to_vec(),
        })
    }

    pub fn get_status_line(&self) -> String {
        format!("{:?} {}\r\n", self.version, self.status)
    }

    pub fn get_headers(&self) -> String {
        self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .chain(std::iter::once("\r\n".to_string()))
            .collect::<String>()
    }
}
