//! Conversation specification

use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};

use crate::model::common::AccountID;
use crate::model::message::Message;

/// The base model for a conversation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Conversation(pub Vec<ConversationEntry>);

/// An item of a conversation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
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
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::basic::Basic;
    use crate::model::conversation::Conversation;

    #[test]
    fn test_deserialize_conversation_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
            "data": [
                {
                    "id": 188129,
                    "with_account": "jasdev",
                    "with_account_id": 3698510,
                    "last_message_preview": "hi",
                    "message_count": 3,
                    "datetime": 1406927327
                }
            ],
            "success": true,
            "status": 200
        }"#;

        let data = serde_json::from_str::<Basic<Conversation>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }
}
