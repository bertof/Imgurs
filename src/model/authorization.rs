//! Authentication data

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::common::{AccountID, AccountUsername};
use crate::serialization::unix_epoch;

/// User access token
///
/// Is your secret key used to access the user's data.
/// It can be thought of the user's password and username combined into one, and is used to access
/// the user's account.
/// It expires after 1 month
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessToken(pub String);

/// Refresh token
///
/// Is used to request new access_tokens.
/// Since access_tokens expire after 1 month, we need a way to request new ones without going
/// through the entire authorization step again.
/// It does not expire.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshToken(pub String);

/// Authorization code
///
/// Is used for obtaining the the access and refresh tokens.
/// It's purpose is to be immediately exchanged for an access_token and refresh_token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationCode(pub String);

/// PIN code
///
/// Is also used for obtaining the the access and refresh tokens, but it's presented to the user so
/// that they can enter it directly into your app.
/// It's purpose is to be immediately exchanged for an access_token and refresh_token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PINCode(pub String);

/// Type of the obtained token
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenType(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationResponse {
    access_token: AccessToken,
    account_id: AccountID,
    account_username: AccountUsername,
    #[serde(with = "unix_epoch")]
    expires_in: DateTime<Utc>,
    refresh_token: RefreshToken,
    scope: serde_json::Value,
    token_type: TokenType,
}

