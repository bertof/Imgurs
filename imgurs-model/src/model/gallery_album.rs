//! Gallery album specification
use super::{common::AccountID, image::Image};
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
    pub cover: Option<String>,
    /// The width, in pixels, of the album cover image
    pub cover_width: Option<u64>,
    /// The height, in pixels, of the album cover image
    pub cover_height: Option<u64>,
    /// The account username or `null` if it's anonymous.
    pub account_url: Option<String>,
    /// The account ID of the account that uploaded it, or `null`.
    pub account_id: Option<AccountID>,
    /// The privacy level of the album, you can only view public if not logged in as album owner
    pub privacy: String,
    /// The view layout of the album.
    pub layout: String,
    /// The number of image views
    pub views: u64,
    /// The URL link to the album
    pub link: Url,
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
    /// The current user's vote on the album. `null` if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// Indicates if the current user favorited the album. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates if the album has been marked as nsfw or not. Defaults to `null` if information is not available.
    pub nsfw: Option<bool>,
    /// Number of comments on the gallery album.
    pub comment_count: u64,
    /// Topic of the gallery album.
    pub topic: Option<String>,
    /// Topic ID of the gallery album.
    pub topic_id: Option<u64>,
    /// The total number of images in the album
    pub images_count: u64,
    /// An array of all the images in the album (only available when requesting the direct album)
    pub images: Option<Vec<Image>>,
    /// Indicates if the album is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,

    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {

    use crate::model::{basic::Basic, gallery_album::GalleryAlbum};
    use time::macros::datetime;

    #[test]
    fn test_deserialize_gallery_album_example() {
        let res = include_str!("../../model_data/gallery_album.example.json");
        let data = serde_json::from_str::<GalleryAlbum>(res).unwrap();
        println!("{:#?}", data);

        assert_eq!(data.account_id.unwrap(), 67659037);
        assert_eq!(data.account_url.unwrap(), "BeanMugged");
        assert_eq!(data.comment_count, 108);
        assert_eq!(data.cover.unwrap(), "MDCEW6Q");
        assert_eq!(data.cover_height.unwrap(), 532);
        assert_eq!(data.cover_width.unwrap(), 513);
        assert_eq!(data.datetime, datetime!(2020-10-19 8:18:58.0 UTC));
        assert_eq!(data.description, None);
        assert_eq!(data.downs, 56);
        assert_eq!(data.favorite.unwrap(), false);
        assert_eq!(data.id, "HvCcoNA");
        assert_eq!(data.images_count, 50);
        assert_eq!(data.in_most_viral.unwrap(), true);
        assert_eq!(data.is_album, true);
        assert_eq!(data.layout, "blog");
        assert_eq!(data.link.to_string(), "https://imgur.com/a/HvCcoNA");
        assert_eq!(data.nsfw.unwrap(), true);
        assert_eq!(data.points, 1267);
        assert_eq!(data.privacy, "hidden");
        assert_eq!(data.score, 1283);
        assert_eq!(data.title.unwrap(), "Dunn's Dumb Dump");
        assert_eq!(data.topic, None);
        assert_eq!(data.topic_id, None);
        assert_eq!(data.ups, 1323);
        assert_eq!(data.views, 32276);
        assert_eq!(data.vote, None);
    }

    #[test]
    fn test_deserialize_gallery_album_real() {
        let res = include_str!("../../model_data/gallery_album.real.json");
        let data = serde_json::from_str::<Basic<GalleryAlbum>>(res).unwrap();
        println!("{:#?}", data);
    }
}
