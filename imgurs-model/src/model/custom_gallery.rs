//! Custom gallery specification
use serde::{Deserialize, Serialize};
use url::Url;

use crate::model::{common::Username, gallery_album::GalleryAlbum, gallery_image::GalleryImage};

/// Custom gallery
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct CustomGallery {
    /// Author
    pub account_url: Username,
    /// Link to the custom gallery
    pub link: Url,
    /// Tags
    pub tags: Vec<String>,
    /// Size of the gallery
    pub item_count: u64,
    /// Contents of the gallery
    pub items: Vec<CustomGalleryItem>,
}

/// Custom gallery item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum CustomGalleryItem {
    /// Gallery image
    GalleryImage(GalleryImage),
    /// Gallery album
    GalleryAlbum(GalleryAlbum),
}

#[cfg(test)]
mod test {

    use crate::model::{basic::Basic, custom_gallery::CustomGallery};

    // TODO: use a better example
    #[test]
    fn test_deserialize_custom_gallery_local() {
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
        let data = serde_json::from_str::<Basic<CustomGallery>>(res).unwrap();
        println!("{:#?}", data);
    }
}
