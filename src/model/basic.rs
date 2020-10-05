//! API response specification

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// API response common fields
///
/// This is the basic response for requests that do not return data. If the POST request has a Basic model it will return the id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Basic<T> {
    /// Response data
    pub data: BasicData<T>,
    /// Success status
    pub success: bool,
    /// HTTP status
    pub status: u16,
}

/// API response result wrapper
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum BasicData<T> {
    /// Content in the response
    Content(T),
    /// Error in the response
    Error {
        /// Error message
        error: String,
        /// Requested content
        request: String,
        /// Method used in the request
        method: Method,
    },
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

    use reqwest::StatusCode;

    use crate::model::{
        account_settings::AccountSettings,
        basic::{Basic, BasicData, Method},
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
            BasicData::Content(_) => panic!("Should return error"),
            BasicData::Error { error, request, method } => {
                assert_eq!(error, "Authentication required");
                assert_eq!(request, "/3/account/me/settings");
                assert_eq!(method, Method::GET);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_error_parsing_remote() -> Result<(), Box<dyn Error>> {
        let res = reqwest::get("https://api.imgur.com/3/account/me/settings").await?;
        assert_eq!(res.status(), StatusCode::from_u16(401)?);

        let data = res.json::<Basic<AccountSettings>>().await?;
        assert!(!data.success);
        assert_eq!(data.status, 401);
        match data.data {
            BasicData::Content(_) => panic!("Should return error"),
            BasicData::Error { error, request, method } => {
                assert_eq!(error, "Authentication required");
                assert_eq!(request, "/3/account/me/settings");
                assert_eq!(method, Method::GET);
            }
        }

        Ok(())
    }
}