//! Authentication data

use std::convert::TryFrom;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::ErrorMessage,
    model::{
        common::AccountID,
        common::Username,
    },
    serialization::unix_epoch,
    traits::FromEnv,
};

/// Client ID
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientID(pub String);

impl TryFrom<String> for ClientID {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(ClientID(value))
    }
}

impl FromEnv for ClientID {
    fn default_env() -> &'static str {
        "CLIENT_ID"
    }
}

impl fmt::Display for ClientID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Client secret
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientSecret(pub String);

impl TryFrom<String> for ClientSecret {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(ClientSecret(value))
    }
}

impl FromEnv for ClientSecret {
    fn default_env() -> &'static str {
        "CLIENT_SECRET"
    }
}

impl fmt::Display for ClientSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// User access token
///
/// Is your secret key used to access the user's data.
/// It can be thought of the user's password and username combined into one, and is used to access
/// the user's account.
/// It expires after 1 month
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessToken(pub String);

impl TryFrom<String> for AccessToken {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(AccessToken(value))
    }
}

impl FromEnv for AccessToken {
    fn default_env() -> &'static str {
        "ACCESS_TOKEN"
    }
}

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Refresh token
///
/// Is used to request new access_tokens.
/// Since access_tokens expire after 1 month, we need a way to request new ones without going
/// through the entire authorization step again.
/// It does not expire.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshToken(pub String);

impl TryFrom<String> for RefreshToken {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(RefreshToken(value))
    }
}

impl FromEnv for RefreshToken {
    fn default_env() -> &'static str {
        "REFRESH_TOKEN"
    }
}

impl fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Authorization code
///
/// Is used for obtaining the the access and refresh tokens.
/// It's purpose is to be immediately exchanged for an access_token and refresh_token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationCode(pub String);

impl TryFrom<String> for AuthorizationCode {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(AuthorizationCode(value))
    }
}

impl FromEnv for AuthorizationCode {
    fn default_env() -> &'static str {
        "AUTHORIZATION_CODE"
    }
}

impl fmt::Display for AuthorizationCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// PIN code
///
/// Is also used for obtaining the the access and refresh tokens, but it's presented to the user so
/// that they can enter it directly into your app.
/// It's purpose is to be immediately exchanged for an access_token and refresh_token.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PINCode(pub String);

impl TryFrom<String> for PINCode {
    type Error = ErrorMessage;

    fn try_from(value: String) -> Result<Self, Self::Error> {

        // TODO: input checks

        if value.is_empty() {
            return Err(ErrorMessage::new("Invalid length"));
        }

        Ok(PINCode(value))
    }
}

impl FromEnv for PINCode {
    fn default_env() -> &'static str {
        "PIN_CODE"
    }
}

impl fmt::Display for PINCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of the obtained token
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenType(pub String);

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Authorization API response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationResponse {
    /// Access token
    pub access_token: AccessToken,
    /// Account id
    pub account_id: AccountID,
    /// Account username
    pub account_username: Username,
    /// Access token expiration date
    #[serde(with = "unix_epoch")]
    pub expires_in: DateTime<Utc>,
    /// Refresh token
    pub refresh_token: RefreshToken,
    /// TODO: missing from API model
    pub scope: serde_json::Value,
    /// Type of the token received
    pub token_type: TokenType,
}

/// Refresh token API response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshResponse {
    /// Access token
    pub access_token: AccessToken,
    /// Account id
    pub account_id: AccountID,
    /// Account username
    pub account_username: Username,
    /// Access token expiration date
    #[serde(with = "unix_epoch")]
    pub expires_in: DateTime<Utc>,
    /// Refresh token
    pub refresh_token: RefreshToken,
    /// TODO: missing from API model
    pub scope: serde_json::Value,
    /// Type of the token received
    pub token_type: TokenType,
}

