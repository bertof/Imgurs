use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::{
    gallery_album::GalleryAlbum,
    gallery_image::GalleryImage,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct CustomGallery {
    pub account_url: String,
    pub link: Url,
    pub tags: Vec<String>,
    pub item_count: u64,
    pub items: Vec<CustomGalleryItem>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub enum CustomGalleryItem {
    GalleryImage(GalleryImage),
    GalleryAlbum(GalleryAlbum),
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{
        model::{basic::Basic},
    };
    use crate::model::custom_gallery::CustomGallery;

    // TODO: use a better example
    #[test]
    fn test_deserialize_custom_gallery_local() -> Result<(), Box<dyn Error>> {
        let res = r#"{
            "data": {
                "link": "https://imgur.com/g/wRBsA",
                "tags" : ["cats", "dogs"],
                "account_url": "jasdev",
                "items": [],
                "item_count": 0
            },
            "success": true,
            "status": 200
        }"#;

        let data = serde_json::from_str::<Basic<CustomGallery>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_custom_gallery_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
}