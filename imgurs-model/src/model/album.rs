//! Album specification

use super::common::AccountID;
use serde::{Deserialize, Serialize};
use std::fmt;
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

use super::image::Image;

/// Album unique identifier
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct AlbumID(String);

impl<U> From<U> for AlbumID
where
    U: Into<String>,
{
    fn from(v: U) -> Self {
        AlbumID(v.into())
    }
}

impl fmt::Display for AlbumID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The base model for an album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Album {
    /// The ID for the album
    pub id: AlbumID,
    /// The title of the album in the gallery
    pub title: Option<String>,
    /// The description of the album in the gallery
    pub description: Option<String>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// The ID of the album cover image
    pub cover: Option<String>,
    /// The width, in pixels, of the album cover image
    pub cover_width: Option<u64>,
    /// The height, in pixels, of the album cover image
    pub cover_height: Option<u64>,
    /// The account username or null if it's anonymous.
    pub account_url: Option<String>,
    /// The account ID or null if it's anonymous.
    pub account_id: Option<AccountID>,
    /// The privacy level of the album, you can only view public if not logged in as album owner
    /// TODO: Switch to enum
    pub privacy: String,
    /// The view layout of the album.
    /// TODO: Switch to enum
    pub layout: String,
    /// The number of album views
    pub views: u64,
    /// The URL link to the album
    pub link: Url,
    /// Indicates if the current user favorited the image. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates if the image has been marked as nsfw or not. Defaults to null if information is not available.
    pub nsfw: Option<bool>,
    /// If the image has been categorized by our backend then this will contain the section the image belongs in. (funny, cats, adviceanimals, wtf, etc)
    pub section: Option<String>,
    /// Order number of the album on the user's album page (defaults to 0 if their albums haven't been reordered)
    pub order: Option<u64>,
    /// OPTIONAL, the deletehash, if you're logged in as the album owner
    #[serde(rename = "deletehash")]
    pub delete_hash: Option<String>,
    /// The total number of images in the album
    pub images_count: Option<u64>,
    /// An array of all the images in the album (only available when requesting the direct album)
    pub images: Vec<Image>,
    /// True if the image has been submitted to the gallery, false if otherwise.
    pub in_gallery: Option<bool>,
}

#[cfg(test)]
mod test {
    use crate::model::{
        album::{Album, AlbumID},
        basic::DataModelAdapter,
    };
    use time::macros::datetime;

    #[test]
    fn test_deserialize_album_example() {
        let res = include_str!("../../model_data/album.example.json");
        let album = serde_json::from_str::<DataModelAdapter<Album>>(res)
            .unwrap()
            .data;
        assert_eq!(album.id, AlbumID::from("lDRB2"));
        assert_eq!(album.title.unwrap(), "Imgur Office");
        assert_eq!(album.description, None);
        assert_eq!(album.cover.unwrap(), "24nLu");
        assert_eq!(album.cover_width, None);
        assert_eq!(album.cover_height, None);
        assert_eq!(album.account_url.unwrap(), "Alan");
        assert_eq!(album.account_id, Some(4));
        assert_eq!(album.privacy, "public");
        assert_eq!(album.layout, "blog");
        assert_eq!(album.views, 13780);
        assert_eq!(album.link.to_string(), "http://alanbox.imgur.com/a/lDRB2");
        assert_eq!(album.favorite, None);
        assert_eq!(album.nsfw, None);
        assert_eq!(album.section, None);
        assert_eq!(album.images_count.unwrap(), 11);
        assert_eq!(album.images_count.unwrap(), album.images.len() as u64);
        assert_eq!(album.in_gallery, None);

        let image = &album.images[0];
        assert_eq!(image.id, "24nLu");
        assert_eq!(image.title, None);
        assert_eq!(image.description, None);
        assert_eq!(image.datetime, datetime!(2013-01-10 22:19:12.0 UTC));
        assert_eq!(image.mime_type, "image/jpeg".to_string());
        assert!(!image.animated);
        assert_eq!(image.width, 2592);
        assert_eq!(image.height, 1944);
        assert_eq!(image.size, 855658);
        assert_eq!(image.views, 135772);
        assert_eq!(image.bandwidth, 116174397976);
        assert_eq!(image.vote, None);
        assert_eq!(image.favorite, None);
        assert_eq!(image.nsfw, None);
        assert_eq!(image.section, None);
        assert_eq!(
            image.link.as_ref().unwrap().to_string(),
            "http://i.imgur.com/24nLu.jpg"
        );
    }
}
