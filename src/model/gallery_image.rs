//! Gallery image specification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::common::AccountID;
use crate::serialization::unix_epoch;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct GalleryImage {
    /// The ID for the image
    pub id: String,
    /// The title of the image.
    pub title: String,
    /// Description of the image.
    pub description: Option<String>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "unix_epoch")]
    pub datetime: DateTime<Utc>,
    /// Image MIME type.
    #[serde(rename = "type")]
    pub mime_type: String,
    /// Is the image animated
    pub animated: bool,
    /// The width of the image in pixels
    pub width: u64,
    /// The height of the image in pixels
    pub height: u64,
    /// The size of the image in bytes
    pub size: u64,
    /// The number of image views
    pub views: u64,
    /// Bandwidth consumed by the image in bytes
    pub bandwidth: u64,
    /// OPTIONAL, the deletehash, if you're logged in as the image owner
    pub deletehash: Option<String>,
    /// The direct link to the the image. (Note: if fetching an animated GIF that was over 20MB in original size, a .gif thumbnail will be returned)
    pub link: Option<Url>,
    /// OPTIONAL, The .gifv link. Only available if the image is animated and type is 'image/gif'.
    pub gifv: Option<Url>,
    /// OPTIONAL, The direct link to the .mp4. Only available if the image is animated and type is 'image/gif'.
    pub mp4: Option<Url>,
    /// OPTIONAL, The Content-Length of the .mp4. Only available if the image is animated and type is 'image/gif'. Note that a zero value (0) is possible if the video has not yet been generated
    pub mp4_size: Option<u64>,
    /// OPTIONAL, Whether the image has a looping animation. Only available if the image is animated and type is 'image/gif'.
    pub looping: Option<bool>,
    /// The current user's vote on the album. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// Indicates if the current user favorited the image. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates if the image has been marked as nsfw or not. Defaults to null if information is not available.
    pub nsfw: Option<bool>,
    /// Number of comments on the gallery image.
    pub comment_count: u64,
    /// Topic of the gallery image.
    pub topic: Option<String>,
    /// Topic ID of the gallery image.
    pub topic_id: Option<u64>,
    /// If the image has been categorized by our backend then this will contain the section the image belongs in. (funny, cats, adviceanimals, wtf, etc)
    pub section: String,
    /// The username of the account that uploaded it, or null.
    pub account_url: Option<String>,
    /// The account ID of the account that uploaded it, or null.
    pub account_id: Option<AccountID>,
    /// Upvotes for the image
    pub ups: u64,
    /// Number of downvotes for the image
    pub downs: u64,
    /// Upvotes minus downvotes
    pub points: i64,
    /// Imgur popularity score
    pub score: i64,
    /// If it's an album or not
    pub is_album: bool,
    /// Indicates if the image is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::{basic::Basic};
    use crate::model::gallery_image::GalleryImage;

    #[test]
    fn test_deserialize_gallery_image_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
            "data": {
                "id": "OUHDm",
                "title": "My most recent drawing. Spent over 100 hours. I'm pretty proud of it.",
                "description": null,
                "datetime": 1349051625,
                "type": "image/jpeg",
                "animated": false,
                "width": 2490,
                "height": 3025,
                "size": 618969,
                "views": 625622,
                "comment_count": 10,
                "bandwidth": 387240623718,
                "vote": null,
                "section": "pics",
                "account_url": "saponifi3d",
                "account_id": 384077,
                "ups": 1889,
                "downs": 58,
                "points": 1831,
                "score": 18935622,
                "is_album": false
            },
            "success" : true,
            "status" : 200
        }"#;

        let data = serde_json::from_str::<Basic<GalleryImage>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}