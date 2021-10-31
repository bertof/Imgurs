//! API common traits
use chrono::{DateTime, Utc};
use reqwest::{header::HeaderMap, Client as ReqwestClient};

use imgurs_model::model::authorization::AccessToken;

use crate::{
    client::{AuthenticationSettings, ClientSettings},
    error::ClientError,
};

/// Generic client trait
pub trait Client: Clone + Sync {
    /// Get default request headers
    fn get_headers(&self) -> Result<HeaderMap, ClientError>;

    /// Get internal client
    fn get_client(&self) -> &ReqwestClient;

    /// Get client settings
    fn get_settings(&self) -> &ClientSettings;
}

/// Registered client trait
pub trait RegisteredClient: Client {
    /// Get the client authentication settings
    fn get_authentication_settings(&self) -> &AuthenticationSettings;
    /// Update the client authentication token with a fresh one
    fn update_authentication_token(&mut self, access_token: AccessToken, expires_in: DateTime<Utc>);
}
