//! Gallery album specification
use crate::model::{common::AccountID, gallery_image::GalleryImage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

/// Gallery album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GalleryAlbum {
    /// The ID for the image
    pub id: String,
    /// The title of the album in the gallery
    pub title: Option<String>,
    /// The description of the album in the gallery
    pub description: Option<String>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// The ID of the album cover image
    pub cover: String,
    /// The width, in pixels, of the album cover image
    pub cover_width: Option<u64>,
    /// The height, in pixels, of the album cover image
    pub cover_height: Option<u64>,
    /// The account username or `null` if it's anonymous.
    pub account_url: Option<String>,
    /// The account ID of the account that uploaded it, or `null`.
    pub account_id: Option<AccountID>,

    // /// TODO: missing from API model
    // pub ad_config: Option<serde_json::Value>,
    // /// TODO: missing from API model
    // pub ad_type: Option<serde_json::Value>,
    // /// TODO: missing from API model
    // pub ad_url: Option<serde_json::Value>,
    /// TODO: missing from API model
    pub comment_count: u64,
    // /// Number of downvotes for the image
    // pub downs: u64,
    // /// Indicates if the current user favorited the album. Defaults to false if not signed in.
    // pub favorite: Option<bool>,
    // /// Indicates the number of users that have favorited the album.
    // pub favorite_count: Option<u64>,
    // /// An array of all the images in the album (only available when requesting the direct album)
    // pub images: Option<Vec<GalleryImage>>,
    // /// The total number of images in the album
    // pub images_count: u64,
    // /// TODO: missing from API model
    // pub in_gallery: Option<bool>,
    // /// Indicates if the album is in the most viral gallery or not.
    // pub in_most_viral: Option<bool>,
    // /// TODO: missing from API model
    // pub include_album_ads: Option<bool>,
    // /// TODO: missing from API model
    // pub is_ad: Option<bool>,
    // /// If it's an album or not
    // pub is_album: bool,
    // /// The view layout of the album.
    // pub layout: String,
    // /// The URL link to the album
    // pub link: Url,
    // /// Indicates if the album has been marked as nsfw or not. Defaults to `null` if information is not available.
    // pub nsfw: Option<bool>,
    // /// Upvotes minus downvotes
    // pub points: i64,
    // /// The privacy level of the album, you can only view public if not logged in as album owner
    // pub privacy: String,
    // /// Imgur popularity score
    // pub score: i64,
    // /// TODO: missing from API model
    // pub section: Option<serde_json::Value>,
    // /// TODO: missing from API model
    // pub tags: Option<Vec<String>>,
    // /// Topic of the gallery album.
    // pub topic: Option<String>,
    // /// Topic ID of the gallery album.
    // pub topic_id: Option<u64>,
    // /// Upvotes for the image
    // pub ups: u64,
    /// The number of image views
    pub views: u64,
    // /// The current user's vote on the album. `null` if not signed in or if the user hasn't voted on it.
    // pub vote: Option<String>,
    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {

    use crate::model::{basic::Basic, gallery_album::GalleryAlbum};

    #[test]
    fn test_deserialize_gallery_album_local() {
        let res = include_str!("../../model_data/gallery_album.json");
        let data = serde_json::from_str::<Basic<GalleryAlbum>>(res).unwrap();
        println!("{:#?}", data);
    }
}
