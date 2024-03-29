//! API response implementation
use reqwest::header::HeaderMap;

use imgurs_model::model::basic::Basic;

/// API response container
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response<T> {
    /// Parsed data contained in the response
    pub content: Basic<T>,
    /// HTTP headers in the response
    pub headers: HeaderMap,
}
