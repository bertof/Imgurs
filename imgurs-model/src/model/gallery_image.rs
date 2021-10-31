//! Gallery image specification

use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{model::common::AccountID, serialization::unix_epoch};

/// Gallery image unique ID
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageID(String);

impl<U> From<U> for GalleryImageID
where
    U: Into<String>,
{
    fn from(v: U) -> Self {
        GalleryImageID(v.into())
    }
}

impl fmt::Display for GalleryImageID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Gallery image
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
pub struct GalleryImage {
    /// The username of the account that uploaded it, or null.
    pub account_url: Option<String>,
    /// The account ID of the account that uploaded it, or null.
    pub account_id: Option<AccountID>,
    /// TODO: missing from API model
    pub ad_type: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub ad_url: Option<serde_json::Value>,
    /// Is the image animated
    pub animated: bool,
    /// Bandwidth consumed by the image in bytes
    pub bandwidth: u64,
    /// Number of comments on the gallery image.
    pub comment_count: Option<u64>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "unix_epoch")]
    pub datetime: DateTime<Utc>,
    /// OPTIONAL, the deletehash, if you're logged in as the image owner
    #[serde(rename = "deletehash")]
    pub delete_hash: Option<String>,
    /// Description of the image.
    pub description: Option<String>,
    /// Number of downvotes for the image
    pub downs: Option<u64>,
    /// TODO: missing from API model
    pub edited: Option<serde_json::Value>,
    /// Indicates if the current user favorited the image. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates the number of users that have favorited the album.
    pub favorite_count: Option<u64>,
    /// OPTIONAL, The .gifv link. Only available if the image is animated and type is 'image/gif'.
    pub gifv: Option<Url>,
    /// TODO: missing from API model
    pub has_sound: Option<bool>,
    /// The height of the image in pixels
    pub height: u64,
    /// The ID for the image
    pub id: String,
    /// TODO: missing from API model
    pub in_gallery: Option<bool>,
    /// Indicates if the image is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
    /// If it's an album or not
    pub is_album: Option<bool>,
    /// TODO: missing from API model
    pub is_ad: Option<bool>,
    /// The direct link to the the image. (Note: if fetching an animated GIF that was over 20MB in original size, a .gif thumbnail will be returned)
    pub link: Option<Url>,
    /// OPTIONAL, Whether the image has a looping animation. Only available if the image is animated and type is 'image/gif'.
    pub looping: Option<bool>,
    /// Image MIME type.
    #[serde(rename = "type")]
    pub mime_type: Option<String>,
    /// OPTIONAL, The direct link to the .mp4. Only available if the image is animated and type is 'image/gif'.
    pub mp4: Option<Url>,
    /// OPTIONAL, The Content-Length of the .mp4. Only available if the image is animated and type is 'image/gif'. Note that a zero value (0) is possible if the video has not yet been generated
    pub mp4_size: Option<u64>,
    /// Indicates if the image has been marked as nsfw or not. Defaults to null if information is not available.
    pub nsfw: Option<bool>,
    /// Upvotes minus downvotes
    pub points: Option<i64>,
    /// Imgur popularity score
    pub score: Option<i64>,
    /// If the image has been categorized by our backend then this will contain the section the image belongs in. (funny, cats, adviceanimals, wtf, etc)
    pub section: Option<String>,
    /// The size of the image in bytes
    pub size: u64,
    /// The title of the image.
    pub title: Option<String>,
    /// Topic of the gallery image.
    pub topic: Option<String>,
    /// Topic ID of the gallery image.
    pub topic_id: Option<u64>,
    /// Upvotes for the image
    pub ups: Option<u64>,
    /// The number of image views
    pub views: u64,
    /// The current user's vote on the album. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// The width of the image in pixels
    pub width: u64,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::basic::Basic;
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
}
