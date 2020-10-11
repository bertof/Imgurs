//! Album specification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::model::common::AccountID;
use crate::serialization::unix_epoch;

/// The base model for an album
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Album {
    /// The ID for the album
    pub id: String,
    /// The title of the album in the gallery
    pub title: String,
    /// The description of the album in the gallery
    pub description: Option<String>,
    /// Time inserted into the gallery, epoch time
    #[serde(with = "unix_epoch")]
    pub datetime: DateTime<Utc>,
    /// The ID of the album cover image
    pub cover: Option<String>,
    /// TODO: missing from API model
    pub cover_edited: Option<Value>,
    /// The width, in pixels, of the album cover image
    pub cover_width: Option<u64>,
    /// The height, in pixels, of the album cover image
    pub cover_height: Option<u64>,
    /// The account username or null if it's anonymous.
    pub account_url: Option<String>,
    /// The account ID or null if it's anonymous.
    pub account_id: Option<AccountID>,
    /// The privacy level of the album, you can only view public if not logged in as album owner
    // TODO: Switch to enum
    pub privacy: String,
    /// The view layout of the album.
    // TODO: Switch to enum
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
    ///
    /// TODO: switch to image object
    pub images: Vec<Value>,
    /// True if the image has been submitted to the gallery, false if otherwise.
    pub in_gallery: bool,
    /// True if the image is an ad
    pub is_ad: bool,
    /// TODO: missing from API model
    pub include_album_ads: bool,
    /// TODO: missing from API model
    pub is_album: bool,
    /// TODO: missing from API model, switch to an object
    pub ad_config: Value,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{
        api::Client,
        model::{album::Album, basic::Basic},
    };
    use crate::model::authorization::{ClientID, ClientSecret};
    use crate::traits::FromEnv;

    #[test]
    fn test_deserialize_album_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{"data":{"id":"z6B0j","title":"DOOGLE","description":null,"datetime":1515221993,"cover":null,"cover_edited":null,"cover_width":null,"cover_height":null,"account_url":null,"account_id":null,"privacy":"public","layout":"blog","views":84,"link":"https://imgur.com/a/z6B0j","favorite":false,"nsfw":false,"section":"ImgurAlbums","images_count":1,"in_gallery":false,"is_ad":false,"include_album_ads":false,"is_album":true,"images":[{"id":"1nneRbX","title":"DOOGLE","description":"Doogle","datetime":1515221708,"type":"image/png","animated":false,"width":1279,"height":717,"size":379024,"views":8705,"bandwidth":3299403920,"vote":null,"favorite":false,"nsfw":null,"section":null,"account_url":null,"account_id":null,"is_ad":false,"in_most_viral":false,"has_sound":false,"tags":[],"ad_type":0,"ad_url":"","edited":"0","in_gallery":false,"link":"https://i.imgur.com/1nneRbX.png"}],"ad_config":{"safeFlags":["not_in_gallery","subreddit","page_load"],"highRiskFlags":[],"unsafeFlags":["sixth_mod_unsafe"],"wallUnsafeFlags":[],"showsAds":false}},"success":true,"status":200}"#;

        let data = serde_json::from_str::<Basic<Album>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_album_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = Client::new(client_id, client_secret)?;

        let data = client.client
            .get("https://api.imgur.com/3/album/z6B0j")
            .send().await?
            .json::<Basic<Album>>().await?;

        println!("{:?}", data);

        Ok(())
    }
}