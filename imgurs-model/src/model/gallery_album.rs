//! Gallery album specification
use serde::{Deserialize, Serialize};
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

use crate::model::{common::AccountID, gallery_image::GalleryImage};

/// Gallery album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
pub struct GalleryAlbum {
    /// The account ID of the account that uploaded it, or `null`.
    pub account_id: Option<AccountID>,
    /// The account username or `null` if it's anonymous.
    pub account_url: Option<String>,
    /// TODO: missing from API model
    pub ad_config: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub ad_type: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub ad_url: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub comment_count: u64,
    /// The ID of the album cover image
    pub cover: String,
    /// The height, in pixels, of the album cover image
    pub cover_height: Option<u64>,
    /// The width, in pixels, of the album cover image
    pub cover_width: Option<u64>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// The description of the album in the gallery
    pub description: Option<String>,
    /// Number of downvotes for the image
    pub downs: u64,
    /// Indicates if the current user favorited the album. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates the number of users that have favorited the album.
    pub favorite_count: Option<u64>,
    /// The ID for the image
    pub id: String,
    /// An array of all the images in the album (only available when requesting the direct album)
    pub images: Option<Vec<GalleryImage>>,
    /// The total number of images in the album
    pub images_count: u64,
    /// TODO: missing from API model
    pub in_gallery: Option<bool>,
    /// Indicates if the album is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
    /// TODO: missing from API model
    pub include_album_ads: Option<bool>,
    /// TODO: missing from API model
    pub is_ad: Option<bool>,
    /// If it's an album or not
    pub is_album: bool,
    /// The view layout of the album.
    pub layout: String,
    /// The URL link to the album
    pub link: Url,
    /// Indicates if the album has been marked as nsfw or not. Defaults to `null` if information is not available.
    pub nsfw: Option<bool>,
    /// Upvotes minus downvotes
    pub points: i64,
    /// The privacy level of the album, you can only view public if not logged in as album owner
    pub privacy: String,
    /// Imgur popularity score
    pub score: i64,
    /// TODO: missing from API model
    pub section: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub tags: Option<Vec<String>>,
    /// The title of the album in the gallery
    pub title: String,
    /// Topic of the gallery album.
    pub topic: Option<String>,
    /// Topic ID of the gallery album.
    pub topic_id: Option<u64>,
    /// Upvotes for the image
    pub ups: u64,
    /// The number of image views
    pub views: u64,
    /// The current user's vote on the album. `null` if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::{basic::Basic, gallery_album::GalleryAlbum};

    #[test]
    fn test_deserialize_gallery_album_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
          "data": {
            "account_id": 67659037,
            "account_url": "BeanMugged",
            "ad_config": {
              "highRiskFlags": [],
              "safeFlags": [
                "in_gallery",
                "gallery",
                "album"
              ],
              "showsAds": false,
              "unsafeFlags": [
                "onsfw_mod_unsafe",
                "sixth_mod_unsafe",
                "mature"
              ],
              "wallUnsafeFlags": []
            },
            "ad_type": 0,
            "ad_url": "",
            "comment_count": 108,
            "cover": "MDCEW6Q",
            "cover_height": 532,
            "cover_width": 513,
            "datetime": 1603095538,
            "description": null,
            "downs": 54,
            "favorite": false,
            "favorite_count": 315,
            "id": "HvCcoNA",
            "images": [
              {
                "account_id": null,
                "account_url": null,
                "ad_type": 0,
                "ad_url": "",
                "animated": false,
                "bandwidth": 8667309096,
                "comment_count": null,
                "datetime": 1592569542,
                "description": null,
                "downs": null,
                "edited": "0",
                "favorite": false,
                "favorite_count": null,
                "has_sound": false,
                "height": 532,
                "id": "MDCEW6Q",
                "in_gallery": false,
                "in_most_viral": false,
                "is_ad": false,
                "link": "https://i.imgur.com/MDCEW6Q.png",
                "nsfw": null,
                "points": null,
                "score": null,
                "section": null,
                "size": 452412,
                "tags": [],
                "title": null,
                "type": "image/png",
                "ups": null,
                "views": 19158,
                "vote": null,
                "width": 513
              },
              {
                "account_id": null,
                "account_url": null,
                "ad_type": 0,
                "ad_url": "",
                "animated": false,
                "bandwidth": 596525155,
                "comment_count": null,
                "datetime": 1592569543,
                "description": null,
                "downs": null,
                "edited": "0",
                "favorite": false,
                "favorite_count": null,
                "has_sound": false,
                "height": 540,
                "id": "1REuHNL",
                "in_gallery": false,
                "in_most_viral": false,
                "is_ad": false,
                "link": "https://i.imgur.com/1REuHNL.jpg",
                "nsfw": null,
                "points": null,
                "score": null,
                "section": null,
                "size": 32215,
                "tags": [],
                "title": null,
                "type": "image/jpeg",
                "ups": null,
                "views": 18517,
                "vote": null,
                "width": 609
              },
              {
                "account_id": null,
                "account_url": null,
                "ad_type": 0,
                "ad_url": "",
                "animated": false,
                "bandwidth": 468810123,
                "comment_count": null,
                "datetime": 1592569544,
                "description": null,
                "downs": null,
                "edited": "0",
                "favorite": false,
                "favorite_count": null,
                "has_sound": false,
                "height": 232,
                "id": "hp2tIwe",
                "in_gallery": false,
                "in_most_viral": false,
                "is_ad": false,
                "link": "https://i.imgur.com/hp2tIwe.jpg",
                "nsfw": null,
                "points": null,
                "score": null,
                "section": null,
                "size": 30201,
                "tags": [],
                "title": null,
                "type": "image/jpeg",
                "ups": null,
                "views": 15523,
                "vote": null,
                "width": 500
              }
            ],
            "images_count": 50,
            "in_gallery": true,
            "in_most_viral": true,
            "include_album_ads": true,
            "is_ad": false,
            "is_album": true,
            "layout": "blog",
            "link": "https://imgur.com/a/HvCcoNA",
            "nsfw": true,
            "points": 1251,
            "privacy": "hidden",
            "score": 1266,
            "section": "",
            "tags": [],
            "title": "Dunn's Dumb Dump",
            "topic": "No Topic",
            "topic_id": 29,
            "ups": 1305,
            "views": 31958,
            "vote": null
          },
          "status": 200,
          "success": true
        }"#;

        let data = serde_json::from_str::<Basic<GalleryAlbum>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }
}
