//! Comment specification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::common::AccountID;
use crate::serialization::unix_epoch;

/// The base model for a comment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Comment {
    /// The ID for the comment
    pub id: u64,
    /// The ID of the image that the comment is for
    pub image_id: String,
    /// The comment itself.
    pub comment: String,
    /// Username of the author of the comment
    pub author: String,
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
    #[serde(with = "unix_epoch")]
    pub datetime: DateTime<Utc>,
    /// If this is a reply, this will be the value of the comment_id for the caption this a reply for.
    ///
    /// Defaults to 0 if it isn't a reply
    pub parent_id: u64,
    /// Marked true if this caption has been deleted
    pub deleted: bool,
    /// The current user's vote on the comment. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// TODO: missing from API model
    pub platform: String,
    /// TODO: missing from API model
    pub has_admin_badge: bool,
    /// All of the replies for this comment. If there are no replies to the comment then this is an empty set.
    pub children: Vec<Comment>,
}

#[cfg(test)]
mod test {
    use std::env::var;
    use std::error::Error;

    use crate::client::imgur_client;
    use crate::model::basic::Basic;
    use crate::model::comment::Comment;

    #[test]
    fn test_deserialize_comment_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{"data":{"id":1938489503,"image_id":"CRprgNU","comment":"Don’t be coy, what is he making?","author":"TheBeigePhillip","author_id":102217801,"on_album":true,"album_cover":"w2gwdJp","ups":93,"downs":2,"points":91,"datetime":1599059372,"parent_id":0,"deleted":false,"vote":null,"platform":"iphone","has_admin_badge":false,"children":[]},"success":true,"status":200}"#;

        let data = serde_json::from_str::<Basic<Comment>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_comment_remote() -> Result<(), Box<dyn Error>> {
        let client_id = var("CLIENT_ID")?;
        let client = imgur_client(&client_id)?;

        let data = client
            .get("https://api.imgur.com/3/comment/1938633683")
            .send().await?
            .json::<Basic<Comment>>().await?;

        println!("{:?}", data);

        Ok(())
    }
}