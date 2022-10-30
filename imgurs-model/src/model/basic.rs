//! API response specification
use crate::error::ErrorMessage;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestError {
    code: String,
    detail: String,
    id: String,
    status: ErrorMessage,
}

/// API response data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Response<T> {
    /// Content in the response
    Content {
        /// Response data
        data: T,
        /// HTTP status code
        status: u16,
        /// Success status
        success: bool,
    },
    /// Error in the response
    Error {
        /// Error message
        error: ErrorMessage,
        /// Requested content
        request: String,
        /// Method used in the request
        method: Method,
    },
    /// Errors in the request
    Errors { errors: Vec<RequestError> },
}

impl<T> Response<T> {
    pub fn result(self) -> Result<T, ErrorMessage> {
        self.into()
    }
}

impl<T> From<Response<T>> for Result<T, ErrorMessage> {
    fn from(r: Response<T>) -> Self {
        match r {
            Response::Content { data, .. } => Ok(data),
            Response::Error { error, .. } => Err(error),
            Response::Errors { errors } => {
                // todo!();
                Err(errors.iter().map(|e| &e.status).into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::basic::Response;

    #[test]
    fn parsing_example() {
        let res = include_str!("../../model_data/basic.example.json");
        let parsed_data = serde_json::from_str::<Response<bool>>(res).unwrap();
        match parsed_data {
            Response::Content {
                data,
                status,
                success,
            } => {
                assert!(success);
                assert_eq!(status, 200);
                assert!(data)
            }
            _ => panic!(
                "Invalid type, shoud be Content instead got {:?}",
                parsed_data
            ),
        }
    }

    #[test]
    fn parsing_errors() {
        let res = include_str!("../../model_data/too_many_requests.real.json");
        let parsed_data = serde_json::from_str::<Response<bool>>(res).unwrap();
        match parsed_data {
            Response::Errors { errors } => {
                assert_eq!(errors.len(), 1);
                let e = &errors[0];
                assert_eq!(e.code, "429");
                assert_eq!(e.status.0, "Too Many Requests");
            }
            _ => panic!(
                "Invalid type, shoud be Errors instead got {:?}",
                parsed_data
            ),
        }
    }
}
