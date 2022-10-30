//! API response implementation
use crate::error::ClientError;
use imgurs_model::{model::basic::Response, utilities::pretty_json};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

/// API response container
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResponseWrapper<T> {
    /// Parsed data contained in the response
    pub response: Response<T>,
    /// HTTP headers in the response
    pub headers: HeaderMap,
}

pub async fn parse_response_or_error<T: DeserializeOwned>(
    res: reqwest::Response,
) -> Result<ResponseWrapper<T>, ClientError> {
    let headers = res.headers().clone();

    #[cfg(feature = "tracing")]
    let response = {
        let text = res.text().await?;
        let text = pretty_json(&text).unwrap();
        tracing::debug!("Body: {}", text);
        println!("Body: {}", text);
        serde_json::from_str(&text)?
    };
    #[cfg(not(feature = "tracing"))]
    let response = res.json().await?;
    Ok(ResponseWrapper { headers, response })
}
