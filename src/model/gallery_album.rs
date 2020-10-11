//! Gallery album specification
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::common::AccountID;
use crate::model::image::Image;
use crate::serialization::unix_epoch;

/// Gallery album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct GalleryAlbum {
    /// The ID for the image
    pub id: String,
    /// The title of the album in the gallery
    pub title: String,
    /// The description of the album in the gallery
    pub description: Option<String>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "unix_epoch")]
    pub datetime: DateTime<Utc>,
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
    pub images: Vec<Image>,
    /// Indicates if the album is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{
        model::{basic::Basic},
    };
    use crate::model::gallery_album::GalleryAlbum;

    #[test]
    fn test_deserialize_gallery_album_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
            "data": {
                "id": "lDRB2",
                "title": "Imgur Office",
                "description": null,
                "datetime": 1357856292,
                "cover": "24nLu",
                "account_url": "Alan",
                "account_id": 4,
                "privacy": "public",
                "layout": "blog",
                "views": 13780,
                "link": "http://alanbox.imgur.com/a/lDRB2",
                "ups": 1602,
                "downs": 14,
                "points": 1588,
                "score": 1917,
                "is_album": true,
                "vote": null,
                "comment_count": 10,
                "images_count": 11,
                "images": [
                    {
                        "id": "24nLu",
                        "title": null,
                        "description": null,
                        "datetime": 1357856352,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 855658,
                        "views": 135772,
                        "bandwidth": 116174397976,
                        "link": "https://i.imgur.com/24nLu.jpg"
                    },
                    {
                        "id": "Ziz25",
                        "title": null,
                        "description": null,
                        "datetime": 1357856394,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 919391,
                        "views": 135493,
                        "bandwidth": 124571044763,
                        "link": "https://i.imgur.com/Ziz25.jpg"
                    },
                    {
                        "id": "9tzW6",
                        "title": null,
                        "description": null,
                        "datetime": 1357856385,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 655028,
                        "views": 135063,
                        "bandwidth": 88470046764,
                        "link": "https://i.imgur.com/9tzW6.jpg"
                    },
                    {
                        "id": "dFg5u",
                        "title": null,
                        "description": null,
                        "datetime": 1357856378,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 812738,
                        "views": 134704,
                        "bandwidth": 109479059552,
                        "link": "https://i.imgur.com/dFg5u.jpg"
                    },
                    {
                        "id": "oknLx",
                        "title": null,
                        "description": null,
                        "datetime": 1357856338,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 1749,
                        "height": 2332,
                        "size": 717324,
                        "views": 32938,
                        "bandwidth": 23627217912,
                        "link": "https://i.imgur.com/oknLx.jpg"
                    },
                    {
                        "id": "OL6tC",
                        "title": null,
                        "description": null,
                        "datetime": 1357856321,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 1443262,
                        "views": 32346,
                        "bandwidth": 46683752652,
                        "link": "https://i.imgur.com/OL6tC.jpg"
                    },
                    {
                        "id": "cJ9cm",
                        "title": null,
                        "description": null,
                        "datetime": 1357856330,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 544702,
                        "views": 31829,
                        "bandwidth": 17337319958,
                        "link": "https://i.imgur.com/cJ9cm.jpg"
                    },
                    {
                        "id": "7BtPN",
                        "title": null,
                        "description": null,
                        "datetime": 1357856369,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 844863,
                        "views": 31257,
                        "bandwidth": 26407882791,
                        "link": "https://i.imgur.com/7BtPN.jpg"
                    },
                    {
                        "id": "42ib8",
                        "title": null,
                        "description": null,
                        "datetime": 1357856424,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 2592,
                        "height": 1944,
                        "size": 905073,
                        "views": 30945,
                        "bandwidth": 28007483985,
                        "link": "https://i.imgur.com/42ib8.jpg"
                    },
                    {
                        "id": "BbwIx",
                        "title": null,
                        "description": null,
                        "datetime": 1357856360,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 1749,
                        "height": 2332,
                        "size": 662413,
                        "views": 30107,
                        "bandwidth": 19943268191,
                        "link": "https://i.imgur.com/BbwIx.jpg"
                    },
                    {
                        "id": "x7b91",
                        "title": null,
                        "description": null,
                        "datetime": 1357856406,
                        "type": "image/jpeg",
                        "animated": false,
                        "width": 1944,
                        "height": 2592,
                        "size": 618567,
                        "views": 29259,
                        "bandwidth": 18098651853,
                        "link": "https://i.imgur.com/x7b91.jpg"
                    }
                ]
            },
            "success": true,
            "status": 200
        }"#;

        let data = serde_json::from_str::<Basic<GalleryAlbum>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_album_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}