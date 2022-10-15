//! Album specification

use std::{collections::HashMap, fmt};

use crate::model::common::AccountID;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub title: String,
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
    pub favorite: bool,
    /// Indicates if the image has been marked as nsfw or not. Defaults to null if information is not available.
    pub nsfw: bool,
    /// If the image has been categorized by our backend then this will contain the section the image belongs in. (funny, cats, adviceanimals, wtf, etc)
    pub section: String,
    /// Order number of the album on the user's album page (defaults to 0 if their albums haven't been reordered)
    pub order: Option<u64>,
    /// OPTIONAL, the deletehash, if you're logged in as the album owner
    #[serde(rename = "deletehash")]
    pub delete_hash: Option<String>,
    /// The total number of images in the album
    pub images_count: u64,
    /// An array of all the images in the album (only available when requesting the direct album)
    pub images: Vec<Image>,
    /// True if the image has been submitted to the gallery, false if otherwise.
    pub in_gallery: bool,

    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {
    use crate::model::{
        album::{Album, AlbumID},
        basic::Basic,
    };
    use time::macros::datetime;

    #[test]
    fn test_deserialize_album_local() {
        let res = r#"{
          "data": {
            "id": "z6B0j",
            "title": "DOOGLE",
            "description": null,
            "datetime": 1515221993,
            "cover": null,
            "cover_edited": null,
            "cover_width": null,
            "cover_height": null,
            "account_url": null,
            "account_id": null,
            "privacy": "public",
            "layout": "blog",
            "views": 84,
            "link": "https://imgur.com/a/z6B0j",
            "favorite": false,
            "nsfw": false,
            "section": "ImgurAlbums",
            "images_count": 1,
            "in_gallery": false,
            "is_ad": false,
            "include_album_ads": false,
            "is_album": true,
            "images": [
              {
                "id": "1nneRbX",
                "title": "DOOGLE",
                "description": "Doogle",
                "datetime": 1515221708,
                "type": "image/png",
                "animated": false,
                "width": 1279,
                "height": 717,
                "size": 379024,
                "views": 8705,
                "bandwidth": 3299403920,
                "vote": null,
                "favorite": false,
                "nsfw": null,
                "section": null,
                "account_url": null,
                "account_id": null,
                "is_ad": false,
                "in_most_viral": false,
                "has_sound": false,
                "tags": [],
                "ad_type": 0,
                "ad_url": "",
                "edited": "0",
                "in_gallery": false,
                "link": "https://i.imgur.com/1nneRbX.png"
              }
            ],
            "ad_config": {
              "safeFlags": [
                "not_in_gallery",
                "subreddit",
                "page_load"
              ],
              "highRiskFlags": [],
              "unsafeFlags": [
                "sixth_mod_unsafe"
              ],
              "wallUnsafeFlags": [],
              "showsAds": false
            }
          },
          "success": true,
          "status": 200
        }
        "#;
        let album = serde_json::from_str::<Basic<Album>>(res)
            .unwrap()
            .result()
            .unwrap();
        println!("{:#?}", album);
        assert_eq!(album.id, AlbumID::from("z6B0j"));
        assert_eq!(album.title, "DOOGLE");
        assert_eq!(album.description, None);
        assert_eq!(album.cover, None);
        assert_eq!(album.cover_width, None);
        assert_eq!(album.cover_height, None);
        assert_eq!(album.account_url, None);
        assert_eq!(album.account_id, None);
        assert_eq!(album.privacy, "public");
        assert_eq!(album.layout, "blog");
        assert_eq!(album.views, 84);
        assert_eq!(album.link.to_string(), "https://imgur.com/a/z6B0j");
        assert!(!album.favorite);
        assert!(!album.nsfw);
        assert_eq!(album.section, "ImgurAlbums");
        assert_eq!(album.images_count, 1);
        assert_eq!(album.images_count, album.images.len() as u64);
        assert!(!album.in_gallery);

        let image = &album.images[0];
        assert_eq!(image.id, "1nneRbX");
        assert_eq!(image.title, "DOOGLE");
        assert_eq!(image.description, Some("Doogle".to_string()));
        assert_eq!(image.datetime, datetime!(2018-01-06 6:55:08.0 UTC));
        assert_eq!(image.mime_type, "image/png".to_string());
        assert!(!image.animated);
        assert_eq!(image.width, 1279);
        assert_eq!(image.height, 717);
        assert_eq!(image.size, 379024);
        assert_eq!(image.views, 8705);
        assert_eq!(image.bandwidth, 3299403920);
        assert_eq!(image.vote, None);
        assert!(!image.favorite);
        assert_eq!(image.nsfw, None);
        assert_eq!(image.section, None);
        assert_eq!(image.link.to_string(), "https://i.imgur.com/1nneRbX.png");
    }
}
