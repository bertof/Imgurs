//! Custom gallery specification
use super::{common::Username, gallery_album::GalleryAlbum, gallery_image::GalleryImage};
use serde::{Deserialize, Serialize};
use url::Url;

/// Custom gallery
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct CustomGallery {
    /// Author
    pub account_url: Option<Username>,
    /// Link to the custom gallery
    pub link: Url,
    /// Tags
    pub tags: Vec<String>,
    /// Size of the gallery
    pub item_count: Option<u64>,
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

    use crate::model::custom_gallery::{CustomGallery, CustomGalleryItem};

    #[test]
    fn test_deserialize_custom_gallery_example() {
        let res = include_str!("../../model_data/custom_gallery.example.json");
        let data = serde_json::from_str::<CustomGallery>(res).unwrap();
        println!("{:#?}", data);
        assert_eq!(data.account_url.unwrap(), "jasdev");
        assert_eq!(data.link.to_string(), "https://imgur.com/g/wRBsA");
        assert_eq!(data.tags, vec!["cats", "dogs"]);
        assert_eq!(data.item_count, None);
        assert_eq!(data.items.len(), 2);
        assert!(matches!(&data.items[0], CustomGalleryItem::GalleryImage(_)));
        assert!(matches!(&data.items[1], CustomGalleryItem::GalleryAlbum(_)));
    }
}
