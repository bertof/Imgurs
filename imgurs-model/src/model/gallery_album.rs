//! Gallery album specification
use super::{album::Album, image::Image};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Gallery album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GalleryAlbum {
    #[serde(flatten)]
    pub album: Album,
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
    /// Number of comments on the gallery album.
    pub comment_count: u64,
    /// Topic of the gallery album.
    pub topic: Option<String>,
    /// Topic ID of the gallery album.
    pub topic_id: Option<u64>,
    /// Indicates if the album is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
}

#[cfg(test)]
mod test {

    use crate::model::{
        basic::{Basic, DataModelAdapter},
        gallery_album::GalleryAlbum,
    };
    use time::macros::datetime;

    #[test]
    fn test_deserialize_gallery_album_example() {
        let res = include_str!("../../model_data/gallery_album.example.json");
        let data = serde_json::from_str::<DataModelAdapter<GalleryAlbum>>(res)
            .unwrap()
            .data;
        println!("{:#?}", data);

        assert_eq!(data.album.account_id.unwrap(), 67659037);
        assert_eq!(data.album.account_url.unwrap(), "BeanMugged");
        assert_eq!(data.comment_count, 108);
        assert_eq!(data.album.cover.unwrap(), "MDCEW6Q");
        assert_eq!(data.album.cover_height.unwrap(), 532);
        assert_eq!(data.album.cover_width.unwrap(), 513);
        assert_eq!(data.album.datetime, datetime!(2020-10-19 8:18:58.0 UTC));
        assert_eq!(data.album.description, None);
        assert_eq!(data.downs, 56);
        assert_eq!(data.album.favorite, Some(false));
        assert_eq!(data.album.id.to_string(), "HvCcoNA");
        assert_eq!(data.album.images_count, Some(50));
        assert!(data.in_most_viral.unwrap());
        assert!(data.is_album);
        assert_eq!(data.album.layout, "blog");
        assert_eq!(data.album.link.to_string(), "https://imgur.com/a/HvCcoNA");
        assert_eq!(data.album.nsfw, Some(true));
        assert_eq!(data.points, 1267);
        assert_eq!(data.album.privacy, "hidden");
        assert_eq!(data.score, 1283);
        assert_eq!(data.album.title.unwrap(), "Dunn's Dumb Dump");
        assert_eq!(data.topic, None);
        assert_eq!(data.topic_id, None);
        assert_eq!(data.ups, 1323);
        assert_eq!(data.album.views, 32276);
        assert_eq!(data.vote, None);
    }

    #[test]
    fn test_deserialize_gallery_album_real() {
        let res = include_str!("../../model_data/gallery_album.real.json");
        let data = serde_json::from_str::<Basic<DataModelAdapter<GalleryAlbum>>>(res).unwrap();
        println!("{:#?}", data);
    }
}
