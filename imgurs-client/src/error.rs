//! API error implementation
use imgurs_model::error::ErrorMessage;
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};
use thiserror::Error;

/// Client error wrapper
#[derive(Debug, Error)]
pub enum ClientError {
    /// URL error
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    /// Request error
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    /// JSON error
    #[error(transparent)]
    JSONError(#[from] serde_json::Error),
    /// Header value to string conversion error
    #[error(transparent)]
    ToStrError(#[from] ToStrError),
    /// Invalid header name error
    #[error(transparent)]
    InvalidHeaderName(#[from] InvalidHeaderName),
    /// Invalid value name error
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    /// Error message
    #[error(transparent)]
    ErrorMessage(#[from] ErrorMessage),
}
