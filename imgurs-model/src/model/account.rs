//! User account specification

use super::common::{AccountID, ProExpiration, Username};
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

/// Basic account information representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
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

// /// User blocked status
// ///
// /// TODO: could not test, always returns 429 too many requests
// #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
// pub struct BlockedStatus {
//     /// Blocked status
//     pub blocked: bool,
// }

// /// List of blocked accounts
// #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
// pub struct AccountBlocks {
//     /// TODO: Missing from API model
//     pub items: Vec<BlockedAccount>,
//     /// TODO: Missing from API model
//     pub next: Option<serde_json::Value>,
// }

// /// Blocked account
// #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
// pub struct BlockedAccount {
//     /// Account username
//     pub url: Username,
// }

// /// Create block response
// #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
// pub struct BlockResponse {
//     /// Blocked status
//     pub blocked: bool,
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::{basic::Response, common::ProExpiration};
    use time::macros::datetime;

    #[test]
    fn parse_account_example() {
        let data = include_str!("../../model_data/account.example.json");
        let account = serde_json::from_str::<Response<Account>>(data)
            .unwrap()
            .result()
            .unwrap();
        assert_eq!(account.id, 48437714);
        assert_eq!(account.url.unwrap(), "ghostinspector");
        assert_eq!(account.bio, None);
        assert_eq!(account.avatar, None);
        assert_eq!(account.avatar_name, None);
        assert_eq!(account.cover, None);
        assert_eq!(account.cover_name, None);
        assert_eq!(account.reputation, 0.0);
        assert_eq!(account.reputation_name.unwrap(), "Neutral");
        assert_eq!(account.created, datetime!(2016-12-15 22:07:48.0 UTC));
        assert_eq!(account.pro_expiration, ProExpiration::Bool(false));
        assert!(!account.user_follow.unwrap().status);
        assert_eq!(account.is_blocked, None);
    }

    #[test]
    fn parse_account_real() {
        let data = include_str!("../../model_data/account.real.json");
        let account = serde_json::from_str::<Response<Account>>(data)
            .unwrap()
            .result()
            .unwrap();
        assert_eq!(account.id, 57420253);
        assert_eq!(account.url.unwrap(), "bertof");
        assert_eq!(account.bio, None);
        assert_eq!(
            account.avatar.unwrap().to_string(),
            "https://imgur.com/user/bertof/avatar?maxwidth=290"
        );
        assert_eq!(account.avatar_name.unwrap(), "default/B");
        assert_eq!(
            account.cover.unwrap().to_string(),
            "https://imgur.com/user/bertof/cover?maxwidth=2560"
        );
        assert_eq!(account.cover_name.unwrap(), "default/1-space");
        assert_eq!(account.reputation, 4.0);
        assert_eq!(account.reputation_name.unwrap(), "Neutral");
        assert_eq!(account.created, datetime!(2017-03-23 21:48:00.0 UTC));
        assert_eq!(account.pro_expiration, ProExpiration::Bool(false));
        assert!(!account.user_follow.unwrap().status);
        assert!(!account.is_blocked.unwrap());
    }
}
