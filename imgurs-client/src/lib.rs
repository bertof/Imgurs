//! Imgur client implementation
#[deny(clippy::all)]
#[deny(unsafe_code)]
pub mod client;
pub mod error;
pub mod response;
#[cfg(test)]
pub(crate) mod test_utils;
pub mod traits;

pub mod endpoints {
    //! API endpoints
    pub mod account;
    pub mod authorization;
    pub mod gallery;
}
