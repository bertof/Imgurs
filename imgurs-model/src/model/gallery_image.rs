//! Gallery image specification

use super::{
    common::{AccountID, Username},
    image::Image,
};
use serde::{Deserialize, Serialize};
use std::fmt;

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
pub struct GalleryImage {
    /// Fields in common with [Image].
    #[serde(flatten)]
    pub image: Image,

    /// Number of comments on the gallery image.
    comment_count: u64,
    /// The username of the account that uploaded it, or null.
    account_url: Username,
    /// The account ID of the account that uploaded it, or null.
    account_id: AccountID,
    /// Upvotes for the image
    ups: u64,
    /// Number of downvotes for the image
    downs: u64,
    /// Upvotes minus downvotes
    points: u64,
    /// Imgur popularity score
    score: u64,
    /// If it's an album or not
    is_album: bool,
    /// Indicates if the image is in the most viral gallery or not.
    in_most_viral: Option<bool>,
}

#[cfg(test)]
mod test {

    use crate::model::{basic::DataModelAdapter, gallery_image::GalleryImage};
    use time::macros::datetime;

    #[test]
    fn test_deserialize_gallery_image_example() {
        let res = include_str!("../../model_data/gallery_image.example.json");
        let data = serde_json::from_str::<DataModelAdapter<GalleryImage>>(res)
            .unwrap()
            .data;
        assert_eq!(data.image.id, "OUHDm");
        assert_eq!(
            data.image.title.unwrap(),
            "My most recent drawing. Spent over 100 hours. I'm pretty proud of it."
        );
        assert_eq!(data.image.description, None);
        assert_eq!(data.image.datetime, datetime!(2012-10-01 0:33:45.0 UTC));
        assert_eq!(data.image.mime_type, "image/jpeg");
        assert!(!data.image.animated);
        assert_eq!(data.image.width, 2490);
        assert_eq!(data.image.height, 3025);
        assert_eq!(data.image.size, 618969);
        assert_eq!(data.image.views, 625622);
        assert_eq!(data.comment_count, 10);
        assert_eq!(data.image.bandwidth, 387240623718);
        assert_eq!(data.image.vote, None);
        assert_eq!(data.image.section.unwrap(), "pics");
        assert_eq!(data.account_url, "saponifi3d");
        assert_eq!(data.account_id, 384077);
        assert_eq!(data.ups, 1889);
        assert_eq!(data.downs, 58);
        assert_eq!(data.points, 1831);
        assert_eq!(data.score, 18935622);
        assert!(!data.is_album);
    }
}
