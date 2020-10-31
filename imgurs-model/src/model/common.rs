//! Common data objects

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::serialization::unix_epoch;

/// Account username
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Username(String);

impl<U> From<U> for Username where U: Into<String> {
    fn from(v: U) -> Self {
        Username(v.into())
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Unique identifier for an Imgur user
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccountID(u64);

impl From<u64> for AccountID {
    fn from(v: u64) -> Self {
        AccountID(v)
    }
}

impl Into<u64> for AccountID {
    fn into(self) -> u64 {
        self.0
    }
}

impl fmt::Display for AccountID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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


