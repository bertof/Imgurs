//! Authorization API wrapper
use crate::{
    client::{AuthenticatedClient, BasicClient},
    error::ClientError,
    response::Response,
    traits::{Client, RegisteredClient},
};
use async_trait::async_trait;
use imgurs_model::model::{
    authorization::{AuthorizationCode, AuthorizationResponse, PINCode, RefreshResponse},
    basic::{Basic, Data},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tracing::debug;
use url::Url;

/// Client authorization API endpoint
pub const CLIENT_AUTHORIZATION_URL: &str = "https://api.imgur.com/oauth2/authorize";
/// Client authorization via token (pin or authentication code) API endpoint
pub const CLIENT_TOKEN_URL: &str = "https://api.imgur.com/oauth2/token";
/// Client authentication token refresh timeout in minutes
pub const REFRESH_TIMEOUT: i64 = 5;

/// Determines if Imgur returns an authorization_code, a PIN code, or an opaque access_token.
/// If you choose code, then you must immediately exchange the `authorization_code` for an access_token.
/// If you chose `token`, then the `access_token` and `refresh_token` will be given to you in the
/// form of query string parameters attached to your redirect URL, which the user may be able to read.
/// If you chose `pin`, then the user will receive a PIN code that they will enter into your app to
/// complete the authorization process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    /// Request authentication through an authorization code
    AuthorizationCode,
    /// Request authentication through a pin code
    Pin,
    /// Request authentication through an HTTP redirect containing the authentication and refresh codes
    Token,
}

impl Method {
    /// Encode authorization response type to the respective URL parameter name
    pub fn to_url_parameter(&self) -> &str {
        match self {
            Method::AuthorizationCode => "code",
            Method::Token => "token",
            Method::Pin => "pin",
        }
    }
}

async fn parse_response_or_error<T: DeserializeOwned>(
    res: reqwest::Response,
) -> Result<Response<T>, ClientError> {
    let status = res.status();
    let headers = res.headers().clone();

    let text = res.text().await?;

    debug!("Body: {}", text);

    // Parse for a T
    let val = serde_json::from_str(&text);

    // If correct return Response with AuthorizationResponse
    if let Ok(val) = val {
        Ok(Response {
            content: Basic {
                data: Data::Content(val),
                success: status.is_success(),
                status: status.as_u16(),
            },
            headers,
        })
    } else {
        // Parse for a BasicData
        serde_json::from_str(&text)
            .map(|data| Response {
                content: data,
                headers,
            })
            .map_err(Into::into)
    }
}

/// Authentication API client
#[async_trait]
pub trait AuthenticationClient: Client {
    /// Generate a client authentication URL based on its client id and an authentication method
    ///
    /// Determines if Imgur returns an authorization_code, a PIN code, or an opaque access_token.
    /// If you choose code, then you must immediately exchange the authorization_code for an access_token.
    /// If you chose token, then the access_token and refresh_token will be given to you in the form of query string parameters attached to your redirect URL, which the user may be able to read.
    /// If you chose pin, then the user will receive a PIN code that they will enter into your app to complete the authorization process.
    fn get_authentication_url(
        &self,
        method: Method,
        state: Option<&str>,
    ) -> Result<Url, ClientError> {
        let url = format!(
            "{}?response_type={}&client_id={}{}",
            CLIENT_AUTHORIZATION_URL,
            method.to_url_parameter(),
            self.get_settings().client_id.0,
            match state {
                Some(v) => format!("&state={}", v),
                None => "".to_string(),
            }
        );
        Url::parse(&url).map_err(Into::into)
    }

    /// Request client authorization through an authorization code
    async fn authorization_by_authorization_code(
        &self,
        code: AuthorizationCode,
    ) -> Result<Response<AuthorizationResponse>, ClientError> {
        let res = self
            .get_client()
            .post(CLIENT_TOKEN_URL)
            .headers(self.get_headers()?)
            .form(&[
                ("client_id", &self.get_settings().client_id.0),
                ("client_secret", &self.get_settings().client_secret.0),
                ("grant_type", &"authorization_code".to_owned()),
                ("code", &code.0),
            ])
            .send()
            .await?;

        parse_response_or_error(res).await
    }

    /// Request client authorization through a pin code
    async fn authorization_by_pin_code(
        &self,
        code: PINCode,
    ) -> Result<Response<AuthorizationResponse>, ClientError> {
        let res = self
            .get_client()
            .post(CLIENT_TOKEN_URL)
            .headers(self.get_headers()?)
            .form(&[
                ("client_id", &self.get_settings().client_id.0),
                ("client_secret", &self.get_settings().client_secret.0),
                ("grant_type", &"pin".to_owned()),
                ("pin", &code.0),
            ])
            .send()
            .await?;

        parse_response_or_error(res).await
    }
}

/// Registered client authentication API client
#[async_trait]
pub trait AuthenticationRegisteredClient: AuthenticationClient + RegisteredClient {
    /// Refresh the client token
    async fn refresh_token(&self) -> Result<Response<RefreshResponse>, ClientError> {
        let res = self
            .get_client()
            .post(CLIENT_TOKEN_URL)
            .headers(self.get_headers()?)
            .form(&[
                ("client_id", &self.get_settings().client_id.to_string()),
                (
                    "client_secret",
                    &self.get_settings().client_secret.to_string(),
                ),
                (
                    "refresh_token",
                    &self.get_authentication_settings().refresh_token.to_string(),
                ),
                ("grant_type", &"refresh_token".to_string()),
            ])
            .send()
            .await?;

        parse_response_or_error(res).await
    }

    /// Chain refresh of tokens if necessary
    ///
    /// If the authentication token of the client is going to expire within
    /// `REFRESH_TIMEOUT` minutes, a refresh is asked and the token is substituted with
    /// a new one
    async fn with_fresh_tokens(self) -> Result<Self, ClientError> {
        if self.get_authentication_settings().expires_in
            > (OffsetDateTime::now_utc() + Duration::minutes(REFRESH_TIMEOUT))
        {
            Ok(self)
        } else {
            let mut client = self.clone();
            let res = client.refresh_token().await?.content.result()?;
            client.update_authentication_token(res.access_token, res.expires_in);
            Ok(client)
        }
    }
}

impl AuthenticationClient for BasicClient {}

impl AuthenticationClient for AuthenticatedClient {}

impl AuthenticationRegisteredClient for AuthenticatedClient {}

#[cfg(test)]
mod tests {
    use crate::{
        client::BasicClient,
        endpoints::authorization::{AuthenticationClient, AuthenticationRegisteredClient, Method},
    };
    use imgurs_model::model::authorization::{
        AccessToken, AuthorizationCode, ClientID, ClientSecret, PINCode, RefreshToken,
    };
    use std::{convert::TryFrom, env, error::Error};
    use time::OffsetDateTime;

    #[test]
    fn test_get_authentication_url_with_authorization_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::AuthorizationCode, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_pin_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::Pin, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_token() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::Token, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_state() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let res =
            client.get_authentication_url(Method::AuthorizationCode, Some("Example state"))?;
        println!("{:?}", res);
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_consume_authorization_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let authorization_code = AuthorizationCode::try_from(env::var("REFRESH_TOKEN")?)?;
        let res = client
            .authorization_by_authorization_code(authorization_code)
            .await?;
        println!("{:?}", res);
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_consume_pin_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let client = BasicClient::new(client_id, client_secret)?;
        let pin_code = PINCode::try_from(env::var("PIN_CODE")?)?;
        println!("{:?}", pin_code);
        let res = client.authorization_by_pin_code(pin_code).await?;
        println!("{:?}", res);
        assert!(res.content.result().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_token() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
        let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
        let access_token = AccessToken::try_from(env::var("ACCESS_TOKEN")?)?;
        let refresh_token = RefreshToken::try_from(env::var("REFRESH_TOKEN")?)?;
        let client = BasicClient::new(client_id, client_secret)?.with_tokens(
            access_token,
            refresh_token,
            OffsetDateTime::now_utc(),
        )?;
        let res = client.refresh_token().await?;
        println!("{:?}", res);
        assert!(res.content.result().is_ok());

        Ok(())
    }
}
