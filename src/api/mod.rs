//! API wrapper

use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::model::authorization::{AccessToken, ClientID, ClientSecret, RefreshToken};
use crate::serialization::unix_epoch;

pub mod error;
pub mod response;
pub mod authorization;

/// Client basic settings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClientSettings {
    /// Client unique ID
    client_id: ClientID,
    /// Client secret token
    client_secret: ClientSecret,
}

/// Client API authentication related settings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationSettings {
    access_token: AccessToken,
    refresh_token: RefreshToken,
    #[serde(with = "unix_epoch")]
    expires_in: DateTime<Utc>,
}

/// Client
///
/// Basic Imgur client, without authentication
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) client: ReqwestClient,
    settings: ClientSettings,
}

/// Authenticated client
///
/// Imgur client, supports authenticated only API endpoints
#[derive(Clone, Debug)]
pub struct AuthenticatedClient {
    pub(crate) client: ReqwestClient,
    settings: ClientSettings,
    authentication: AuthenticationSettings,
}

impl Client {
    /// `Client` constructor
    pub fn new(client_id: ClientID, client_secret: ClientSecret) -> Result<Self, Box<dyn Error>> {
        let client = ReqwestClientBuilder::new()
            .default_headers(vec![
                (HeaderName::from_str("Authorization")?,
                 HeaderValue::from_str(&format!("Client-ID {}", client_id.0))?)
            ].iter().cloned().collect())
            .build()?;

        // TODO: input checks

        let settings = ClientSettings {
            client_id,
            client_secret,
        };

        Ok(Client {
            client,
            settings,
        })
    }

    /// `AuthenticatedClient` constructor from a `Client`
    pub fn with_tokens<C>(self, access_token: AccessToken, refresh_token: RefreshToken, expires_in: C) -> Result<AuthenticatedClient, Box<dyn Error>>
        where C: Into<DateTime<Utc>> {

        // TODO: input checks

        let authentication = AuthenticationSettings {
            access_token,
            refresh_token,
            expires_in: expires_in.into(),
        };

        Ok(AuthenticatedClient {
            client: self.client,
            settings: self.settings,
            authentication,
        })
    }
}




