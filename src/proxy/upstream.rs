use reqwest::Response;

use crate::{error::ProxyError, proxy::request::HttpRequest};

pub async fn send_req(
    req: &HttpRequest,
    client: &reqwest::Client,
    target: &str,
) -> Result<Response, ProxyError> {
    let url = format!("{}{}", target, req.path);
    let req_type = client.request(reqwest::Method::try_from(&req.method)?, url);
    let body = req_type.send().await?;
    Ok(body)
}
