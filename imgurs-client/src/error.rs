//! API error implementation

use std::{error::Error, fmt};

use reqwest::header::{InvalidHeaderName, InvalidHeaderValue, ToStrError};

use imgurs_model::error::ErrorMessage;

/// Client error wrapper
#[derive(Debug)]
pub enum ClientError {
    /// URL error
    UrlParseError(url::ParseError),
    /// Request error
    RequestError(reqwest::Error),
    /// JSON error
    JSONError(serde_json::Error),
    /// Header value to string conversion error
    ToStrError(ToStrError),
    /// Invalid header name error
    InvalidHeaderName(InvalidHeaderName),
    /// Invalid value name error
    InvalidHeaderValue(InvalidHeaderValue),
    /// Error message
    ErrorMessage(ErrorMessage),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::UrlParseError(e) => e.fmt(f),
            ClientError::RequestError(e) => e.fmt(f),
            ClientError::JSONError(e) => e.fmt(f),
            ClientError::ToStrError(e) => e.fmt(f),
            ClientError::InvalidHeaderName(e) => e.fmt(f),
            ClientError::InvalidHeaderValue(e) => e.fmt(f),
            ClientError::ErrorMessage(e) => e.fmt(f),
        }
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

impl From<InvalidHeaderName> for ClientError {
    fn from(e: InvalidHeaderName) -> Self {
        ClientError::InvalidHeaderName(e)
    }
}

impl From<InvalidHeaderValue> for ClientError {
    fn from(e: InvalidHeaderValue) -> Self {
        ClientError::InvalidHeaderValue(e)
    }
}

impl From<ErrorMessage> for ClientError {
    fn from(e: ErrorMessage) -> Self {
        ClientError::ErrorMessage(e)
    }
}
