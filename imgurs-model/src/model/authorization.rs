//! Authentication data

use crate::model::common::{AccessToken, AccountID, RefreshToken, Username};
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};

/// Type of authorization token
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum TokenType {
    Bearer,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_generate_access_token_real() {
        let res = include_str!("../../model_data/generate_access_token.real.json");
        serde_json::from_str::<AuthorizationResponse>(res).unwrap();
    }
}
