//! Gallery profile specification

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::serialization::unix_epoch;

/// The totals for a users gallery information.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct GalleryProfile {
    /// Total number of comments the user has made in the gallery
    total_gallery_comments: u64,
    /// Total number of items favorited by the user in the gallery
    total_gallery_favorites: Option<u64>,
    /// TODO: missing from API model
    total_gallery_likes: Option<u64>,
    /// Total number of images submitted by the user.
    total_gallery_submissions: u64,
    /// An array of trophies that the user has.
    trophies: Vec<Trophy>,
}

/// Gallery trophy
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct Trophy {
    /// The ID of the trophy, this is unique to each trophy
    id: u64,
    /// The name of the trophy
    name: String,
    /// Can be thought of as the ID of a trophy type
    name_clean: String,
    /// A description of the trophy and how it was earned.
    description: String,
    /// The ID of the image or the ID of the comment where the trophy was earned
    data: Option<String>,
    /// A link to where the trophy was earned
    data_link: Option<Url>,
    /// Date the trophy was earned, epoch time
    #[serde(with = "unix_epoch")]
    datetime: DateTime<Utc>,
    /// URL of the image representing the trophy
    image: Url,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::model::basic::Basic;
    use crate::model::gallery_profile::GalleryProfile;

    #[test]
    fn test_deserialize_gallery_profile_local() -> Result<(), Box<dyn Error>> {
        let res = {
            r#"{
            "data": {
                "total_gallery_comments": 40,
                "total_gallery_likes": 23,
                "total_gallery_submissions": 4,
                "trophies": [
                    {
                        "id": 1,
                        "name": "1 Year",
                        "name_clean": "1Years",
                        "description": "Be a member of Imgur for one year.",
                        "data": null,
                        "data_link": null,
                        "datetime": 1357344455,
                        "image": "https://s.imgur.com/images/trophies/a84ade.png"
                    }
                ]
            },
            "success": true,
            "status": 200
        }"#
        };

        let data = serde_json::from_str::<Basic<GalleryProfile>>(res)?;

        println!("{:#?}", data);

        Ok(())
    }
}
