//! API response implementation
use crate::error::ClientError;
use imgurs_model::{
    model::basic::{Basic, Data},
    utilities::pretty_json,
};
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

/// API response container
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response<T> {
    /// Parsed data contained in the response
    pub content: Basic<T>,
    /// HTTP headers in the response
    pub headers: HeaderMap,
}

pub async fn parse_response_or_error<T: DeserializeOwned>(
    res: reqwest::Response,
) -> Result<Response<T>, ClientError> {
    let status = res.status();
    let headers = res.headers().clone();

    #[cfg(feature = "tracing")]
    let val = {
        let text = res.text().await?;
        // let text = pretty_json(&text).unwrap();
        tracing::debug!("Body: {}", text);
        // println!("Body: {}", text);
        serde_json::from_str(&text)?
    };
    #[cfg(not(feature = "tracing"))]
    let val = res.json().await?;

    let content = Basic {
        data: Data::Content(val),
        success: status.is_success(),
        status: status.as_u16(),
    };

    Ok(Response { content, headers })
}
