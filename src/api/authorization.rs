//! Authorization API wrapper

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tracing::debug;
use url::Url;

use crate::{
    api::{
        AuthenticatedClient,
        Client,
        error::ClientError,
        response::Response,
    },
    model::{
        authorization::{AuthorizationCode, AuthorizationResponse, PINCode, RefreshResponse},
        basic::{Basic, Data},
    },
};

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

async fn parse_response_or_error<T: DeserializeOwned>(res: reqwest::Response) -> Result<Response<T>, ClientError> {
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

impl Client {
    /// Generate a client authentication URL based on its client id and an authentication method
    ///
    /// Determines if Imgur returns an authorization_code, a PIN code, or an opaque access_token.
    /// If you choose code, then you must immediately exchange the authorization_code for an access_token.
    /// If you chose token, then the access_token and refresh_token will be given to you in the form of query string parameters attached to your redirect URL, which the user may be able to read.
    /// If you chose pin, then the user will receive a PIN code that they will enter into your app to complete the authorization process.
    pub fn get_authentication_url(&self, method: Method, state: Option<&str>) -> Result<Url, ClientError> {
        let url = format!(
            "{}?response_type={}&client_id={}{}",
            CLIENT_AUTHORIZATION_URL,
            method.to_url_parameter(),
            self.settings.client_id.0,
            match state {
                Some(v) => format!("&state={}", v),
                None => "".to_string()
            }
        );
        Url::parse(&url).map_err(Into::into)
    }

    /// Request client authorization through an authorization code
    pub async fn authorization_by_authorization_code(&self, code: AuthorizationCode) -> Result<Response<AuthorizationResponse>, ClientError> {
        let res = self.client
            .post(CLIENT_TOKEN_URL)
            .form(&[
                ("client_id", &self.settings.client_id.0),
                ("client_secret", &self.settings.client_secret.0),
                ("grant_type", &"authorization_code".to_owned()),
                ("code", &code.0)])
            .send().await?;

        parse_response_or_error(res).await
    }

    /// Request client authorization through a pin code
    pub async fn authorization_by_pin_code(&self, code: PINCode) -> Result<Response<AuthorizationResponse>, ClientError> {
        let res = self.client
            .post(CLIENT_TOKEN_URL)
            .form(&[
                ("client_id", &self.settings.client_id.0),
                ("client_secret", &self.settings.client_secret.0),
                ("grant_type", &"pin".to_owned()),
                ("pin", &code.0)])
            .send().await?;

        parse_response_or_error(res).await
    }
}

impl AuthenticatedClient {
    /// Refresh the client token
    pub async fn refresh_token(&mut self) -> Result<Response<RefreshResponse>, ClientError> {
        let res = self.client
            .post(CLIENT_TOKEN_URL)
            .form(&[
                ("client_id", &self.settings.client_id.0),
                ("client_secret", &self.settings.client_secret.0),
                ("refresh_token", &self.authentication.refresh_token.0),
                ("grant_type", &"refresh_token".to_string())])
            .send().await?;

        parse_response_or_error(res).await
    }

    /// Chain refresh of tokens if necessary
    ///
    /// If the authentication token of the client is going to expire within
    /// `REFRESH_TIMEOUT` minutes, a refresh is asked and the token is substituted with
    /// a new one
    pub async fn with_fresh_tokens(self) -> Result<Self, ClientError> {
        if self.authentication.expires_in > Utc::now()
            .checked_add_signed(Duration::minutes(REFRESH_TIMEOUT)).unwrap() {
            Ok(self)
        } else {
            let mut client = self.clone();
            client.refresh_token().await?;
            Ok(client)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use chrono::Utc;

    use crate::api::{
        authorization::Method,
        Client,
    };
    use crate::model::authorization::{AccessToken, AuthorizationCode, ClientID, ClientSecret, PINCode, RefreshToken};
    use crate::traits::FromEnv;

    #[test]
    fn test_get_authentication_url_with_authorization_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::AuthorizationCode, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_pin_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::Pin, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_token() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::Token, None)?;
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_state() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let res = client.get_authentication_url(Method::AuthorizationCode, Some("Example state"))?;
        println!("{:?}", res);
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_consume_authorization_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let authorization_code = AuthorizationCode::from_default_env()?;
        let res = client
            .authorization_by_authorization_code(authorization_code).await?;
        println!("{:?}", res);
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_consume_pin_code() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;
        let pin_code = PINCode::from_default_env()?;
        println!("{:?}", pin_code);
        let res = client
            .authorization_by_pin_code(pin_code).await?;
        println!("{:?}", res);
        assert!(res.content.result().is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_token() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let access_token = AccessToken::from_default_env()?;
        let refresh_token = RefreshToken::from_default_env()?;
        let mut client = Client::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?;
        let res = client.refresh_token().await?;
        println!("{:?}", res);
        assert!(res.content.result().is_ok());

        Ok(())
    }
}