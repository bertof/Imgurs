//! User account specification

use super::common::{AccountID, ProExpiration, Username};
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

/// Basic account information representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// The account id for the username requested.
    pub id: AccountID,
    /// The account username, will be the same as requested in the URL
    pub url: Option<Username>,
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
    pub reputation_name: Option<String>,
    /// The epoch time of account creation
    #[serde(with = "timestamp")]
    pub created: OffsetDateTime,
    /// False if not a pro user, their expiration date if they are.
    pub pro_expiration: ProExpiration,
    /// Blocked status
    pub is_blocked: Option<bool>,
    /// User follow status
    pub user_follow: Option<UserFollow>,
}

/// User follow status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct UserFollow {
    /// Following status
    pub status: bool,
}

/// User blocked status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BlockedStatus {
    /// Blocked status
    pub blocked: bool,
}

/// List of blocked accounts
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountBlocks {
    /// TODO: Missing from API model
    pub items: Vec<BlockedAccount>,
    /// TODO: Missing from API model
    pub next: Option<serde_json::Value>,
}

/// Blocked account
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BlockedAccount {
    /// Account username
    pub url: Username,
}

/// Create block response
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BlockResponse {
    /// Blocked status
    pub blocked: bool,
}

#[cfg(test)]
mod test {
    use crate::model::{account::Account, common::ProExpiration};
    use time::macros::datetime;

    #[test]
    fn test_deserialize_account_example() {
        let data = include_str!("../../model_data/account.example.json");
        let account = serde_json::from_str::<Account>(data).unwrap();
        assert_eq!(account.id, 384077);
        assert_eq!(account.url.unwrap(), "joshTest");
        assert_eq!(
            account.bio.unwrap(),
            "A real hoopy frood who really knows where his towel is at."
        );
        assert_eq!(account.avatar, None);
        assert_eq!(account.avatar_name, None);
        assert_eq!(account.cover, None);
        assert_eq!(account.cover_name, None);
        assert_eq!(account.reputation, 15303.84);
        assert_eq!(account.reputation_name, None);
        assert_eq!(account.created, datetime!(2013-08-19 22:31:44.0 UTC));
        assert_eq!(account.pro_expiration, ProExpiration::Bool(false));
        assert_eq!(account.user_follow, None);
        assert_eq!(account.is_blocked, None);
    }
}
