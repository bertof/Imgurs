//! Message specification
use super::common::{AccountID, Username};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{serde::timestamp, OffsetDateTime};

/// The base model for a message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Message {
    /// The ID for the message
    id: u64,
    /// Account username of person sending the message
    from: Username,
    /// The account ID of the person receiving the message
    account_id: AccountID,
    /// The account ID of the person who sent the message
    sender_id: AccountID,
    /// Text of the message
    body: String,
    /// ID for the overall conversation
    conversation_id: u64,
    /// Time message was sent, epoch time
    #[serde(with = "timestamp")]
    datetime: OffsetDateTime,
}
