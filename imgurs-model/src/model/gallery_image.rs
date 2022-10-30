//! Gallery image specification

use super::{
    common::{AccountID, Username},
    image::Image,
};
use serde::{Deserialize, Serialize};

pub type AlbumImages = Vec<GalleryImage>;

pub type GalleryImageID = String;

/// Gallery image
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GalleryImage {
    /// Fields in common with [Image].
    #[serde(flatten)]
    pub image: Image,

    /// The account ID of the account that uploaded it, or null.
    pub account_id: Option<AccountID>,
    /// The username of the account that uploaded it, or null.
    pub account_url: Option<Username>,
    /// Indicates if the image is in the most viral gallery or not.
    pub in_most_viral: Option<bool>,
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::basic::Response;
    use time::macros::datetime;

    #[test]
    fn parse_gallery_image_example() {
        let res = include_str!("../../model_data/gallery_image.example.json");
        let image = serde_json::from_str::<GalleryImage>(res).unwrap();
        let expected_image = GalleryImage {
            account_id: None,
            account_url: None,
            in_most_viral: Some(false),
            image: Image {
                id: "42ib8".to_string(),
                title: None,
                description: None,
                datetime: datetime!(2013-01-10 22:20:24.0 UTC),
                mime_type: "image/jpeg".to_string(),
                animated: false,
                width: 2592,
                height: 1944,
                size: 905073,
                views: 106297,
                bandwidth: 96206544681,
                link: Some("https://i.imgur.com/42ib8.jpg".parse().unwrap()),
                deletehash: None,
                favorite: Some(false),
                gifv: None,
                in_gallery: Some(false),
                looping: None,
                mp4: None,
                mp4_size: None,
                name: None,
                nsfw: None,
                section: None,
                vote: None,
            },
        };
        assert_eq!(image, expected_image);
    }

    #[test]
    fn parse_account_images_real() {
        let res_data = include_str!("../../model_data/account_images.real.json");
        let parsed_data = serde_json::from_str::<Response<Vec<GalleryImage>>>(res_data)
            .unwrap()
            .result()
            .unwrap();
        assert_eq!(parsed_data.len(), 1);
        let image = parsed_data.first().unwrap();

        let expected_image = GalleryImage {
            image: Image {
                id: "vnKe11t".to_string(),
                title: None,
                description: None,
                datetime: datetime!(2020-11-05 0:09:33.0 UTC),
                mime_type: "image/jpeg".to_string(),
                animated: false,
                width: 225,
                height: 225,
                size: 3653,
                views: 2,
                bandwidth: 7306,
                deletehash: Some("111111111111111".to_string()),
                name: Some("italian hand meme.jpeg".to_string()),
                section: None,
                link: Some("https://i.imgur.com/vnKe11t.jpg".parse().unwrap()),
                gifv: None,
                mp4: None,
                mp4_size: None,
                looping: None,
                favorite: Some(false),
                nsfw: None,
                vote: None,
                in_gallery: Some(false),
            },
            account_id: Some(57420253),
            account_url: Some("bertof".to_string()),
            in_most_viral: Some(false),
        };

        assert_eq!(*image, expected_image);
    }
}
