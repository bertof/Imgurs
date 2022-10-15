//! Account settings specification

use serde::{Deserialize, Serialize};

use crate::model::common::{AccountID, ProExpiration};

/// The account settings, only accessible if you're logged in as the user.
///
/// Only accessible if you're logged in as the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct AccountSettings {
    /// The account username
    account_url: Option<String>,
    /// The users email address
    email: String,
    /// Automatically allow all images to be publicly accessible
    public_images: bool,
    /// Set the album privacy to this privacy setting on creation
    /// TODO: switch to enum
    album_privacy: String,
    /// False if not a pro user, their expiration date if they are.
    pro_expiration: ProExpiration,
    /// True if the user has accepted the terms of uploading to the Imgur gallery.
    accepted_gallery_terms: bool,
    /// The email addresses that have been activated to allow uploading
    active_emails: Vec<String>,
    /// If the user is accepting incoming messages or not
    messaging_enabled: bool,
    /// An array of users that have been blocked from messaging, the object is blocked_id and blocked_url.
    blocked_users: Vec<BlockedUser>,
    /// True if the user has opted to have mature images displayed in gallery list endpoints.
    show_mature: bool,
    /// True unless the user created their account via a third party service such as Google Plus.
    first_party: bool,
}

/// Blocked user entry
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct BlockedUser {
    /// Blocked user ID
    pub blocked_id: AccountID,
    /// Blocked user username
    pub blocked_url: String,
}

#[cfg(test)]
mod test {
    use crate::model::{
        account_settings::{AccountSettings, BlockedUser},
        basic::Basic,
        common::ProExpiration,
    };
    use std::error::Error;

    #[test]
    fn test_deserialize_account_settings_local() -> Result<(), Box<dyn Error>> {
        let data = r#"{
            "data": {
                "email": "josh@imgur.com",
                "public_images": false,
                "album_privacy": "secret",
                "pro_expiration": false,
                "accepted_gallery_terms": true,
                "active_emails": [],
                "messaging_enabled": true,
                "blocked_users": [{
                    "blocked_id" : 384077,
                    "blocked_url": "joshTest"
                }],
                "show_mature": false,
                "first_party": true
            },
            "success": true,
            "status": 200
        }"#;
        let account_settings = serde_json::from_str::<Basic<AccountSettings>>(data)?.result()?;
        println!("{:#?}", account_settings);
        assert_eq!(account_settings.email, "josh@imgur.com");
        assert!(!account_settings.public_images);
        assert_eq!(account_settings.album_privacy, "secret");
        assert_eq!(account_settings.pro_expiration, ProExpiration::Bool(false));
        assert!(account_settings.accepted_gallery_terms);
        assert_eq!(account_settings.active_emails, Vec::<String>::new());
        assert!(account_settings.messaging_enabled);
        assert_eq!(
            account_settings.blocked_users,
            vec![BlockedUser {
                blocked_id: 384077,
                blocked_url: "joshTest".to_string()
            }]
        );
        assert!(!account_settings.show_mature);
        assert!(account_settings.first_party);
        Ok(())
    }
}
