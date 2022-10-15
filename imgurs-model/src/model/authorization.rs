//! Authentication data

use crate::{
    error::ErrorMessage,
    model::{common::AccountID, common::Username},
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use time::{serde::timestamp, OffsetDateTime};

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

impl fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Authorization code
///
/// Is used for obtaining the the access and refresh tokens.
/// It's purpose is to be immediately exchanged for an access_token and refresh_token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl fmt::Display for PINCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of the obtained token
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenType(pub String);

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Authorization API response
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationResponse {
    /// Access token
    pub access_token: AccessToken,
    /// Account id
    pub account_id: AccountID,
    /// Account username
    pub account_username: Username,
    /// Access token expiration date
    #[serde(with = "timestamp")]
    pub expires_in: OffsetDateTime,
    /// Refresh token
    pub refresh_token: RefreshToken,
    /// TODO: missing from API model
    pub scope: serde_json::Value,
    /// Type of the token received
    pub token_type: TokenType,
}

/// Refresh token API response
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshResponse {
    /// Access token
    pub access_token: AccessToken,
    /// Account id
    pub account_id: AccountID,
    /// Account username
    pub account_username: Username,
    /// Access token expiration date
    #[serde(with = "timestamp")]
    pub expires_in: OffsetDateTime,
    /// Refresh token
    pub refresh_token: RefreshToken,
    /// TODO: missing from API model
    pub scope: serde_json::Value,
    /// Type of the token received
    pub token_type: TokenType,
}
