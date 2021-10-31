//! Error module
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// String based error message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorMessage(pub String);

impl ErrorMessage {
    /// `ErrorMessage` constructor
    pub fn new<T>(message: T) -> Self
    where
        T: Into<String>,
    {
        ErrorMessage(message.into())
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ErrorMessage {}
