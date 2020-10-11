//! API error implementation

use std::error::Error;
use std::fmt;

use http::header::ToStrError;

/// Client error wrapper
#[derive(Debug)]
pub enum ClientError {
    /// URL error
    UrlParseError(url::ParseError),
    /// Request error
    RequestError(reqwest::Error),
    /// JSON error
    JSONError(serde_json::Error),
    ///
    ToStrError(ToStrError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ClientError::UrlParseError(e) => e.to_string(),
            ClientError::RequestError(e) => e.to_string(),
            ClientError::JSONError(e) => e.to_string(),
            ClientError::ToStrError(e) => e.to_string()
        })
    }
}

impl Error for ClientError {}

impl From<url::ParseError> for ClientError {
    fn from(e: url::ParseError) -> Self {
        ClientError::UrlParseError(e)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::RequestError(e)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError::JSONError(e)
    }
}

impl From<ToStrError> for ClientError {
    fn from(e: ToStrError) -> Self {
        ClientError::ToStrError(e)
    }
}