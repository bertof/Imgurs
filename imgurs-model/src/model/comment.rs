//! Comment specification

use super::common::{AccountID, Username};
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};

/// The base model for a comment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Comment {
    /// The ID for the comment
    pub id: u64,
    /// The ID of the image that the comment is for
    pub image_id: String,
    /// The comment itself.
    pub comment: Option<String>,
    /// Username of the author of the comment
    ///
    /// WARNING: Unlike stated in the API model, [Comment::author] may be `NULL`.
    pub author: Option<Username>,
    /// The account ID for the author
    pub author_id: AccountID,
    /// If this comment was done to an album
    pub on_album: bool,
    /// The ID of the album cover image, this is what should be displayed for album comments
    pub album_cover: Option<String>,
    /// Number of upvotes for the comment
    pub ups: u64,
    /// The number of downvotes for the comment
    pub downs: u64,
    /// the number of upvotes - downvotes
    pub points: f64,
    /// Timestamp of creation, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// If this is a reply, this will be the value of the comment_id for the caption this a reply for.
    ///
    /// Defaults to 0 if it isn't a reply
    pub parent_id: Option<u64>,
    /// Marked true if this caption has been deleted
    pub deleted: bool,
    /// The current user's vote on the comment. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// All of the replies for this comment. If there are no replies to the comment then this is an empty set.
    pub children: Option<Vec<Comment>>,
}

#[cfg(test)]
mod test {
    use crate::model::{basic::DataModelAdapter, comment::Comment};
    use time::macros::datetime;

    #[test]
    fn test_deserialize_comment_example() {
        let res = include_str!("../../model_data/comment.example.json");
        let data = serde_json::from_str::<DataModelAdapter<Comment>>(res)
            .unwrap()
            .data;
        assert_eq!(data.id, 1110);
        assert_eq!(data.image_id, "K84kO");
        assert_eq!(data.comment, None);
        assert_eq!(data.author, Some("joshTest".to_string()));
        assert_eq!(data.author_id, 384077);
        assert!(!data.on_album);
        assert_eq!(data.album_cover, None);
        assert_eq!(data.ups, 5);
        assert_eq!(data.downs, 0);
        assert_eq!(data.points, 5.0);
        assert_eq!(data.datetime, datetime!(2012-09-07 0:24:47.0 UTC));
        assert_eq!(data.parent_id, None);
        assert!(data.deleted);
        assert_eq!(data.vote, None);
        assert_eq!(data.children, None);
    }
}
