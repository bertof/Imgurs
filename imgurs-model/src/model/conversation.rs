//! Conversation specification

use super::{common::AccountID, message::Message};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::{serde::timestamp, OffsetDateTime};

/// The base model for a conversation.
pub type Conversation = Vec<ConversationEntry>;

/// An item of a conversation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConversationEntry {
    /// Conversation ID
    pub id: u64,
    /// Preview of the last message
    ///
    /// TODO: check if Option
    pub last_message_preview: String,
    /// Time of last sent message, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// Account ID of the other user in conversation
    pub with_account_id: AccountID,
    /// Account username of the other user in conversation
    pub with_account: String,
    /// Total number of messages in the conversation
    pub message_count: u64,
    /// OPTIONAL: (only available when requesting a specific conversation) Reverse sorted such that most recent message is at the end of the array.
    pub messages: Option<Vec<Message>>,
    /// OPTIONAL: (only available when requesting a specific conversation) Flag to indicate whether you've reached the beginning of the thread.
    pub done: Option<bool>,
    /// OPTIONAL: (only available when requesting a specific conversation) Number of the next page
    pub page: Option<u64>,

    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {
    use crate::model::conversation::Conversation;
    use time::macros::datetime;

    #[test]
    fn test_deserialize_conversation_example() {
        let res = include_str!("../../model_data/conversation.example.json");
        let data = serde_json::from_str::<Conversation>(res).unwrap();
        let message = &data[0];
        assert_eq!(message.id, 188129);
        assert_eq!(message.with_account, "jasdev");
        assert_eq!(message.with_account_id, 3698510);
        assert_eq!(message.last_message_preview, "hi");
        assert_eq!(message.message_count, 3);
        assert_eq!(message.datetime, datetime!(2014-08-01 21:08:47.0 UTC));
    }
}
