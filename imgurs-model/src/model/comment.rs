//! Comment specification

use std::collections::HashMap;

use crate::model::common::{AccountID, Username};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub comment: String,
    /// Username of the author of the comment
    ///
    /// WARNING: Unlike stated in the API model, [Comment::author] may be `NULL`.
    pub author: Option<Username>,
    /// The account ID for the author
    pub author_id: AccountID,
    /// If this comment was done to an album
    pub on_album: bool,
    /// The ID of the album cover image, this is what should be displayed for album comments
    pub album_cover: String,
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
    pub parent_id: u64,
    /// Marked true if this caption has been deleted
    pub deleted: bool,
    /// The current user's vote on the comment. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// All of the replies for this comment. If there are no replies to the comment then this is an empty set.
    pub children: Vec<Comment>,

    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {
    use time::macros::datetime;

    use crate::model::{basic::Basic, comment::Comment};
    #[test]
    fn test_deserialize_comment_local() {
        let res = r#"{
            "data": {
                "id": 1938489503,
                "image_id": "CRprgNU",
                "comment": "Don’t be coy, what is he making?",
                "author": "TheBeigePhillip",
                "author_id": 102217801,
                "on_album": true,
                "album_cover": "w2gwdJp",
                "ups": 93,
                "downs": 2,
                "points": 91,
                "datetime": 1599059372,
                "parent_id": 0,
                "deleted": false,
                "vote": null,
                "platform": "iphone",
                "has_admin_badge": false,
                "children": []
            },
            "success": true,
            "status": 200
        }"#;
        let data = serde_json::from_str::<Basic<Comment>>(res)
            .unwrap()
            .result()
            .unwrap();
        println!("{:#?}", data);
        assert_eq!(data.id, 1938489503);
        assert_eq!(data.image_id, "CRprgNU");
        assert_eq!(data.comment, "Don’t be coy, what is he making?");
        assert_eq!(data.author, Some("TheBeigePhillip".to_string()));
        assert_eq!(data.author_id, 102217801);
        assert!(data.on_album);
        assert_eq!(data.album_cover, "w2gwdJp");
        assert_eq!(data.ups, 93);
        assert_eq!(data.downs, 2);
        assert_eq!(data.points, 91.0);
        assert_eq!(data.datetime, datetime!(2020-9-2 15:09:32 UTC));
        assert_eq!(data.parent_id, 0);
        assert!(!data.deleted);
        assert_eq!(data.vote, None);
        assert_eq!(data.children, vec![]);
    }
}
