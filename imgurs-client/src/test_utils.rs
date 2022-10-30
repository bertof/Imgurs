use crate::client::{AuthenticatedClient, BasicClient};
use imgurs_model::model::common::{AccessToken, ClientID, ClientSecret, RefreshToken};
pub use std::{convert::TryFrom, env};
use time::OffsetDateTime;

pub fn get_basic_client() -> Result<BasicClient, Box<dyn std::error::Error>> {
    let client_id = ClientID::try_from(env::var("CLIENT_ID")?)?;
    let client_secret = ClientSecret::try_from(env::var("CLIENT_SECRET")?)?;
    BasicClient::new(client_id, client_secret)
}

pub fn get_authenticated_client() -> Result<AuthenticatedClient, Box<dyn std::error::Error>> {
    let access_token = AccessToken::try_from(env::var("ACCESS_TOKEN")?)?;
    let refresh_token = RefreshToken::try_from(env::var("REFRESH_TOKEN")?)?;
    get_basic_client()?.with_tokens(access_token, refresh_token, OffsetDateTime::now_utc())
}
