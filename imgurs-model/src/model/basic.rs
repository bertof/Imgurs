//! API response specification
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ErrorMessage;

/// HTTP methods
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(deny_unknown_fields)]
pub enum Method {
    /// Get method
    GET,
    /// Post method
    POST,
    /// Put method
    PUT,
    /// Delete method
    DELETE,
}

/// API response data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Data<T> {
    /// Content in the response
    Content(T),
    /// Error in the response
    Error {
        /// Error message
        error: ErrorMessage,
        /// Requested content
        request: String,
        /// Method used in the request
        method: Method,
    },
}

impl<T> From<Data<T>> for Result<T, ErrorMessage> {
    fn from(d: Data<T>) -> Self {
        match d {
            Data::Content(c) => Ok(c),
            Data::Error {
                error,
                request: _,
                method: _,
            } => Err(error),
        }
    }
}

/// Data model adapter
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DataModelAdapter<T> {
    #[serde(flatten)]
    pub data: T,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

/// API response common fields
///
/// This is the basic response for requests that do not return data. If the POST request has a Basic model it will return the id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Basic<T> {
    /// Response data.
    ///
    /// Is null, boolean, or integer value. If it's a post then this will contain an object with the all
    /// generated values, such as an ID.
    pub data: Data<T>,
    /// Success status.
    ///
    /// Was the request successful
    pub success: bool,
    /// HTTP status code
    pub status: u16,
}

impl<T> From<Basic<T>> for Result<T, ErrorMessage> {
    fn from(b: Basic<T>) -> Self {
        b.data.into()
    }
}

impl<T> Basic<T> {
    /// Convert Basic into a result
    pub fn result(self) -> Result<T, ErrorMessage> {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use crate::model::basic::Basic;

    #[test]
    fn test_error_parsing_example() {
        let res = include_str!("../../model_data/basic.example.json");
        let data = serde_json::from_str::<Basic<bool>>(res).unwrap();
        assert!(data.success);
        assert_eq!(data.status, 200);
        assert!(data.result().unwrap());
    }
}
