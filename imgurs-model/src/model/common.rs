//! Common data objects
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};

/// Account username
pub type Username = String;

/// Unique identifier for an Imgur user
pub type AccountID = u64;

/// Pro status expiration
///
/// `Boolean(false)` if not a pro user, their expiration date if they are.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ProExpiration {
    /// Expiration date of the pro status as unix timestamp
    #[serde(with = "timestamp")]
    Date(OffsetDateTime),
    /// Boolean of whether the account has a pro status
    Bool(bool),
}
