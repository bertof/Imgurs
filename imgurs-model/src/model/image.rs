//! Image specification

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{serde::timestamp, OffsetDateTime};
use url::Url;

/// The base model for an image.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    /// The ID for the image
    pub id: String,
    /// The title of the image.
    pub title: Option<String>,
    /// Description of the image.
    pub description: Option<String>,
    /// Time uploaded, epoch time
    #[serde(with = "timestamp")]
    pub datetime: OffsetDateTime,
    /// Image MIME type.
    #[serde(rename = "type")]
    pub mime_type: String,
    /// Is the image animated
    pub animated: bool,
    /// The width of the image in pixels
    pub width: u64,
    /// The height of the image in pixels
    pub height: u64,
    /// The size of the image in bytes
    pub size: u64,
    /// The number of image views
    pub views: u64,
    /// Bandwidth consumed by the image in bytes
    pub bandwidth: u64,
    /// OPTIONAL, the deletehash, if you're logged in as the image owner
    pub deletehash: Option<String>,
    /// OPTIONAL, the original filename, if you're logged in as the image owner
    pub name: Option<String>,
    /// If the image has been categorized by our backend then this will contain the section the image belongs in. (funny, cats, adviceanimals, wtf, etc)
    pub section: Option<String>,
    /// The direct link to the the image. (Note: if fetching an animated GIF that was over 20MB in original size, a .gif thumbnail will be returned)
    pub link: Option<Url>,
    /// OPTIONAL, The .gifv link. Only available if the image is animated and type is 'image/gif'.
    pub gifv: Option<Url>,
    /// OPTIONAL, The direct link to the .mp4. Only available if the image is animated and type is 'image/gif'.
    pub mp4: Option<Url>,
    /// OPTIONAL, The Content-Length of the .mp4. Only available if the image is animated and type is 'image/gif'. Note that a zero value (0) is possible if the video has not yet been generated
    pub mp4_size: Option<u64>,
    /// OPTIONAL, Whether the image has a looping animation. Only available if the image is animated and type is 'image/gif'.
    pub looping: Option<bool>,
    /// Indicates if the current user favorited the image. Defaults to false if not signed in.
    pub favorite: Option<bool>,
    /// Indicates if the image has been marked as nsfw or not. Defaults to null if information is not available.
    pub nsfw: Option<bool>,
    /// The current user's vote on the album. null if not signed in or if the user hasn't voted on it.
    pub vote: Option<String>,
    /// True if the image has been submitted to the gallery, false if otherwise.
    in_gallery: Option<bool>,

    /// Other fields that are missing from the API model
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[cfg(test)]
mod test {
    use crate::model::image::Image;
    use time::macros::datetime;

    #[test]
    fn test_deserialize_image_example() {
        let res_data = include_str!("../../model_data/image.example.json");
        let image = serde_json::from_str::<Image>(res_data).unwrap();
        println!("{:#?}", image);
        assert_eq!(image.id, "SbBGk");
        assert_eq!(image.title, None);
        assert_eq!(image.description, None);
        assert_eq!(image.datetime, datetime!(2012-07-06 0:06:33.0 UTC));
        assert_eq!(image.mime_type, "image/jpeg");
        assert!(!image.animated);
        assert_eq!(image.width, 2559);
        assert_eq!(image.height, 1439);
        assert_eq!(image.size, 521916);
        assert_eq!(image.views, 1);
        assert_eq!(image.bandwidth, 521916);
        assert_eq!(image.vote, None);
        assert_eq!(image.favorite, None);
        assert_eq!(image.nsfw, None);
        assert_eq!(image.section, None);
        assert_eq!(
            image.link.unwrap().to_string(),
            "http://i.imgur.com/SbBGk.jpg"
        );
    }
}
