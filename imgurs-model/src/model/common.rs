//! Common data objects
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::serialization::unix_epoch;

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
    /// Expiration date of the pro status
    #[serde(with = "unix_epoch")]
    Date(DateTime<Utc>),
    /// Boolean of whether the account has a pro status
    Bool(bool),
}
