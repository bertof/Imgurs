//! User account specification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::common::{AccountID, ProExpiration};
use crate::serialization::unix_epoch;

/// Basic account information representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// The account id for the username requested.
    pub id: AccountID,
    /// The account username, will be the same as requested in the URL
    pub url: String,
    /// A basic description the user has filled out
    pub bio: Option<String>,
    /// Avatar URL
    pub avatar: Option<Url>,
    /// Avatar name
    pub avatar_name: Option<String>,
    /// Cover image
    pub cover: Option<Url>,
    /// Cover name
    pub cover_name: Option<String>,
    /// The reputation for the account, in it's numerical format.
    pub reputation: f64,
    /// String description of the user reputation
    pub reputation_name: String,
    /// The epoch time of account creation
    #[serde(with = "unix_epoch")]
    pub created: DateTime<Utc>,
    /// False if not a pro user, their expiration date if they are.
    pub pro_expiration: ProExpiration,
    /// Blocked status
    pub is_blocked: bool,
    /// User follow status
    pub user_follow: UserFollow,
}


/// User follow status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct UserFollow {
    status: bool
}


#[cfg(test)]
mod test {
    use crate::api::Client;
    use crate::model::account::Account;
    use crate::model::authorization::{ClientID, ClientSecret};
    use crate::model::basic::Basic;
    use crate::traits::FromEnv;
    use std::error::Error;

    #[test]
    fn test_deserialize_account_local() -> Result<(), Box<dyn Error>> {
        let data = r#"{"data":{"id":48437714,"url":"ghostinspector","bio":null,"avatar":"https://imgur.com/user/ghostinspector/avatar?maxwidth=290","avatar_name":"default/G","cover":"https://imgur.com/user/ghostinspector/cover?maxwidth=2560","cover_name":"default/1-space","reputation":-252,"reputation_name":"Neutral","created":1481839668,"pro_expiration":false,"user_follow":{"status":false},"is_blocked":false},"success":true,"status":200}"#;

        let account = serde_json::from_str::<Basic<Account>>(data)?;

        println!("{:#?}", account);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_account_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;

        let account = client.client
            .get("https://api.imgur.com/3/account/ghostinspector")
            .send().await?
            .json::<Basic<Account>>().await?;

        println!("{:#?}", account);

        Ok(())
    }
}