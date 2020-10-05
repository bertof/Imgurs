use std::error::Error;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::api::Client;
use crate::model::authorization::{AuthorizationCode, AuthorizationResponse};

pub const CLIENT_AUTHORIZATION_URL: &str = "https://api.imgur.com/oauth2/authorize";
pub const CLIENT_TOKEN_URL: &str = "https://api.imgur.com/oauth2/token";
pub const CLIENT_ID_HEADER_NAME: &str = "Client-ID";

/// Determines if Imgur returns an authorization_code, a PIN code, or an opaque access_token.
/// If you choose code, then you must immediately exchange the `authorization_code` for an access_token.
/// If you chose `token`, then the `access_token` and `refresh_token` will be given to you in the
/// form of query string parameters attached to your redirect URL, which the user may be able to read.
/// If you chose `pin`, then the user will receive a PIN code that they will enter into your app to
/// complete the authorization process.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    AuthorizationCode,
    Token,
    Pin,
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

impl<'a> Client<'a> {
    /// Generate a client authentication URL based on its client id and an authentication method
    ///
    /// Determines if Imgur returns an authorization_code, a PIN code, or an opaque access_token.
    /// If you choose code, then you must immediately exchange the authorization_code for an access_token.
    /// If you chose token, then the access_token and refresh_token will be given to you in the form of query string parameters attached to your redirect URL, which the user may be able to read.
    /// If you chose pin, then the user will receive a PIN code that they will enter into your app to complete the authorization process.
    pub fn get_authentication_url(&self, method: Method, state: Option<&str>) -> Result<Url, Box<dyn Error>> {
        let url = format!(
            "{}?response_type={}&client_id={}{}",
            CLIENT_AUTHORIZATION_URL,
            method.to_url_parameter(),
            self.client_strings.client_id,
            match state {
                Some(v) => format!("&state={}", v),
                None => "".to_string()
            }
        );
        Url::parse(&url).map_err(Into::into)
    }

    pub async fn get_tokens(&self, code: AuthorizationCode) -> Result<(), Box<dyn Error>> {
        let res = self.inner
            .post(CLIENT_TOKEN_URL)
            .form(&[
                ("client_id", self.client_strings.client_id),
                ("client_secret", self.client_strings.client_secret.unwrap()),
                ("grant_type", "authorization_code"),
                ("code", &code.0)])
            .send().await?
            .json::<AuthorizationResponse>().await?;

        println!("{}", serde_json::to_string_pretty(&res)?);

        Ok(())
    }

    // pub async fn code_authentication<'b>(&self, state: Option<&str>) -> Result<Client<'b>, Box<dyn Error>> {
    //     let url = self.get_authentication_url(Method::AuthorizationCode, state)?;
    //
    //     debug!("Url: {}", url);
    //
    //     self.inner.get()
    //
    //     unimplemented!()
    // }
}


#[cfg(test)]
mod tests {
    use std::{
        env::var,
        error::Error,
    };

    use crate::api::{
        authorization::Method,
        Client,
    };
    use crate::model::authorization::AuthorizationCode;

    #[test]
    fn test_get_authentication_url() -> Result<(), Box<dyn Error>> {
        let client_id = var("CLIENT_ID")?;
        let client = Client::new(&client_id, None, None)?;
        println!("{}", client.get_authentication_url(Method::AuthorizationCode, None)?);
        Ok(())
    }

    #[test]
    fn test_get_authentication_url_with_state() -> Result<(), Box<dyn Error>> {
        let client_id = var("CLIENT_ID")?;
        let client = Client::new(&client_id, None, None)?;
        println!("{}", client.get_authentication_url(Method::AuthorizationCode, Some("Example"))?);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_tokens() -> Result<(), Box<dyn Error>> {
        let client_id = var("CLIENT_ID")?;
        let client_secret = var("CLIENT_SECRET")?;
        let client = Client::new(
            &client_id, Some(&client_secret), None)?;
        client.get_tokens(AuthorizationCode("be7b47c8cfbb65517f35aa87ba767abe2cc17213".to_string())).await?;

        Ok(())
    }
}