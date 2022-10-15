//! API response specification
use serde::{Deserialize, Serialize};

use crate::error::ErrorMessage;

/// API response common fields
///
/// This is the basic response for requests that do not return data. If the POST request has a Basic model it will return the id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Basic<T> {
    /// Response data
    pub data: Data<T>,
    /// Success status
    pub success: bool,
    /// HTTP status
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

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::{
        account_settings::AccountSettings,
        basic::{Basic, Data, ErrorMessage, Method},
    };

    #[test]
    fn test_error_parsing_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
            "data": {
                "error": "Authentication required",
                "request": "/3/account/me/settings",
                "method": "GET"
            },
            "success": false,
            "status": 401
        }"#;

        let data = serde_json::from_str::<Basic<AccountSettings>>(res)?;
        assert!(!data.success);
        assert_eq!(data.status, 401);
        match data.data {
            Data::Content(_) => panic!("Should return error"),
            Data::Error {
                error,
                request,
                method,
            } => {
                assert_eq!(error, ErrorMessage::new("Authentication required"));
                assert_eq!(request, "/3/account/me/settings");
                assert_eq!(method, Method::GET);
            }
        }

        Ok(())
    }
}
