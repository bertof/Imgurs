use std::{error::Error, fmt, str::FromStr};

use chrono::{DateTime, Utc};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder,
};
use serde::{Deserialize, Serialize};

use imgurs_model::{
    model::authorization::{AccessToken, ClientID, ClientSecret, RefreshToken},
    serialization::unix_epoch,
};

use crate::{
    error::ClientError,
    traits::{Client, RegisteredClient},
};

/// Client basic settings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClientSettings {
    /// Client unique ID
    pub client_id: ClientID,
    /// Client secret token
    pub client_secret: ClientSecret,
}

/// Client API authentication related settings
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationSettings {
    /// Access token
    pub access_token: AccessToken,
    /// Refresh token
    pub refresh_token: RefreshToken,
    /// Access token expiration
    #[serde(with = "unix_epoch")]
    pub expires_in: DateTime<Utc>,
}

/// Client
///
/// Basic Imgur client, without authentication
#[derive(Clone, Debug)]
pub struct BasicClient {
    client: ReqwestClient,
    settings: ClientSettings,
}

impl BasicClient {
    /// `Client` constructor
    pub fn new(client_id: ClientID, client_secret: ClientSecret) -> Result<Self, Box<dyn Error>> {
        let client = ReqwestClientBuilder::new()
            .default_headers(
                vec![(
                    HeaderName::from_str("Authorization")?,
                    HeaderValue::from_str(&format!("Client-ID {}", client_id.0))?,
                )]
                .iter()
                .cloned()
                .collect(),
            )
            .build()?;

        // TODO: input checks

        let settings = ClientSettings {
            client_id,
            client_secret,
        };

        Ok(BasicClient { client, settings })
    }

    /// `AuthenticatedClient` constructor from a `Client`
    pub fn with_tokens<C>(
        self,
        access_token: AccessToken,
        refresh_token: RefreshToken,
        expires_in: C,
    ) -> Result<AuthenticatedClient, Box<dyn Error>>
    where
        C: Into<DateTime<Utc>>,
    {
        // TODO: input checks

        let authentication = AuthenticationSettings {
            access_token,
            refresh_token,
            expires_in: expires_in.into(),
        };

        Ok(AuthenticatedClient {
            client: self.client,
            client_settings: self.settings,
            authentication_settings: authentication,
        })
    }
}

impl Client for BasicClient {
    fn get_headers(&self) -> Result<HeaderMap, ClientError> {
        Ok([
            (
                HeaderName::from_str("Authorization")?,
                HeaderValue::from_str(&format!("Client-ID {}", self.get_settings().client_id))?,
            ),
            (
                HeaderName::from_str("Accept")?,
                HeaderValue::from_str("application/vnd.api+json")?,
            ),
        ]
        .iter()
        .cloned()
        .collect())
    }

    fn get_client(&self) -> &ReqwestClient {
        &self.client
    }

    fn get_settings(&self) -> &ClientSettings {
        &self.settings
    }
}

/// Authenticated client
///
/// Imgur client, supports authenticated only API endpoints
#[derive(Clone, Debug)]
pub struct AuthenticatedClient {
    client: ReqwestClient,
    client_settings: ClientSettings,
    authentication_settings: AuthenticationSettings,
}

impl Client for AuthenticatedClient {
    fn get_headers(&self) -> Result<HeaderMap, ClientError> {
        Ok([
            (
                HeaderName::from_str("Authorization")?,
                HeaderValue::from_str(&format!(
                    "Bearer {}",
                    self.get_authentication_settings().access_token
                ))?,
            ),
            (
                HeaderName::from_str("Accept")?,
                HeaderValue::from_str("application/vnd.api+json")?,
            ),
        ]
        .iter()
        .cloned()
        .collect())
    }

    fn get_client(&self) -> &ReqwestClient {
        &self.client
    }

    fn get_settings(&self) -> &ClientSettings {
        &self.client_settings
    }
}

impl RegisteredClient for AuthenticatedClient {
    fn get_authentication_settings(&self) -> &AuthenticationSettings {
        &self.authentication_settings
    }

    fn update_authentication_token(
        &mut self,
        access_token: AccessToken,
        expires_in: DateTime<Utc>,
    ) {
        self.authentication_settings.access_token = access_token;
        self.authentication_settings.expires_in = expires_in
    }
}

/// Response contents sorting preference
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortPreference {
    /// Newest to oldest
    Newest,
    /// Oldest to newest
    Oldest,
    /// Best to worst
    Best,
    /// Worst to best
    Worst,
}

impl Default for SortPreference {
    fn default() -> Self {
        SortPreference::Newest
    }
}

impl fmt::Display for SortPreference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SortPreference::Newest => "newest",
                SortPreference::Oldest => "oldest",
                SortPreference::Best => "best",
                SortPreference::Worst => "worst",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use reqwest::StatusCode;

    use imgurs_model::{
        error::ErrorMessage,
        model::{
            account::Account,
            account_settings::AccountSettings,
            album::Album,
            authorization::{ClientID, ClientSecret},
            basic::{Basic, Data, Method},
            comment::Comment,
        },
        traits::from_env::FromEnv,
    };

    use crate::{client::BasicClient, traits::Client};

    #[tokio::test]
    async fn test_deserialize_account_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;

        let account = client
            .get_client()
            .get("https://api.imgur.com/3/account/ghostinspector")
            .send()
            .await?
            .json::<Basic<Account>>()
            .await?;

        println!("{:#?}", account);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_album_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;

        let data = client
            .get_client()
            .get("https://api.imgur.com/3/album/z6B0j")
            .send()
            .await?
            .json::<Basic<Album>>()
            .await?;

        println!("{:?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_comment_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;

        let data = client
            .get_client()
            .get("https://api.imgur.com/3/comment/1938633683")
            .send()
            .await?
            .json::<Basic<Comment>>()
            .await?;

        println!("{:?}", data);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_account_settings_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_conversation_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_custom_gallery_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_profile_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[tokio::test]
    async fn test_error_parsing_remote() -> Result<(), Box<dyn Error>> {
        let res = reqwest::get("https://api.imgur.com/3/account/me/settings").await?;
        assert_eq!(res.status(), StatusCode::from_u16(401)?);

        let data = res.json::<Basic<AccountSettings>>().await?;
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
