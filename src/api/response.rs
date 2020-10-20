//! API response implementation


use reqwest::header::HeaderMap;

use crate::model::basic::Basic;

/// API response container
#[derive(Clone, Debug, PartialEq)]
pub struct Response<T> {
    /// Parsed data contained in the response
    pub content: Basic<T>,
    /// HTTP headers in the response
    pub headers: HeaderMap,
}