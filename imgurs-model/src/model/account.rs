//! User account specification

use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

use crate::model::common::{AccountID, ProExpiration, Username};

/// Basic account information representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// The account id for the username requested.
    pub id: AccountID,
    /// The account username, will be the same as requested in the URL
    pub url: Username,
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
    #[serde(with = "timestamp")]
    pub created: OffsetDateTime,
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
    use crate::model::{
        account::{Account, UserFollow},
        basic::Basic,
        common::ProExpiration,
    };
    use time::macros::datetime;

    #[test]
    fn test_deserialize_account_local() {
        let data = r#"{
          "data": {
            "id": 48437714,
            "url": "ghostinspector",
            "bio": null,
            "avatar": "https://imgur.com/user/ghostinspector/avatar?maxwidth=290",
            "avatar_name": "default/G",
            "cover": "https://imgur.com/user/ghostinspector/cover?maxwidth=2560",
            "cover_name": "default/1-space",
            "reputation": -252,
            "reputation_name": "Neutral",
            "created": 1481839668,
            "pro_expiration": false,
            "user_follow": {
              "status": false
            },
            "is_blocked": false
          },
          "success": true,
          "status": 200
        }
        "#;
        let account = serde_json::from_str::<Basic<Account>>(data)
            .unwrap()
            .result()
            .unwrap();
        println!("{:#?}", account);
        assert_eq!(account.id, 48437714);
        assert_eq!(account.url, "ghostinspector");
        assert_eq!(account.bio, None);
        assert_eq!(
            account.avatar.unwrap().to_string(),
            "https://imgur.com/user/ghostinspector/avatar?maxwidth=290"
        );
        assert_eq!(account.avatar_name.unwrap(), "default/G");
        assert_eq!(
            account.cover.unwrap().to_string(),
            "https://imgur.com/user/ghostinspector/cover?maxwidth=2560"
        );
        assert_eq!(account.cover_name.unwrap(), "default/1-space");
        assert_eq!(account.reputation, -252.0);
        assert_eq!(account.reputation_name, "Neutral");
        assert_eq!(account.created, datetime!(2016-12-15 22:07:48.0 UTC));
        assert_eq!(account.pro_expiration, ProExpiration::Bool(false));
        assert_eq!(account.user_follow, UserFollow { status: false });
        assert!(!account.is_blocked);
    }
}
