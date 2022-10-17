//! Gallery API implementation

use async_trait::async_trait;

use imgurs_model::{
    model::{
        album::AlbumID,
        gallery_album::GalleryAlbum,
        gallery_image::{GalleryImage, GalleryImageID},
        gallery_tags::GalleryTags,
    },
    utilities::pretty_json,
};

use crate::{
    client::{AuthenticatedClient, BasicClient},
    error::ClientError,
    response::Response,
    traits::{Client, RegisteredClient},
};

/// Gallery API client
#[async_trait]
pub trait GalleryClient: Client {
    /// Gallery album
    ///
    /// Get additional information about an album in the gallery.
    async fn get_gallery_album(
        &self,
        album_id: &AlbumID,
    ) -> Result<Response<GalleryAlbum>, ClientError> {
        let res = self
            .get_client()
            .get(&format!(
                "https://api.imgur.com/3/gallery/album/{}",
                album_id
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Gallery image
    ///
    /// Get additional information about an image in the gallery.
    async fn get_gallery_image(
        &self,
        gallery_image_id: &GalleryImageID,
    ) -> Result<Response<GalleryImage>, ClientError> {
        let res = self
            .get_client()
            .get(&format!(
                "https://api.imgur.com/3/gallery/image/{id}",
                id = gallery_image_id
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        println!("{:#?}", headers);

        let text = res.text().await?;
        println!("{}", text);
        println!("{}", pretty_json(&text)?);

        let content = serde_json::from_str(&text)?;
        // let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Gallery image
    ///
    /// Get additional information about an image in the gallery.
    async fn get_gallery_tags(&self) -> Result<Response<GalleryTags>, ClientError> {
        let res = self
            .get_client()
            .get("https://api.imgur.com/3/tags")
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        println!("{:#?}", headers);

        let text = res.text().await?;
        println!("{}", text);
        println!("{}", pretty_json(&text)?);

        let content = serde_json::from_str(&text)?;
        // let content = res.json().await?;

        Ok(Response { content, headers })
    }
}

/// Registered client gallery API client
#[async_trait]
pub trait GalleryRegisteredClient: RegisteredClient {}

impl GalleryClient for BasicClient {}

impl GalleryClient for AuthenticatedClient {}

impl GalleryRegisteredClient for AuthenticatedClient {}

#[cfg(test)]
mod tests {
    use crate::{endpoints::gallery::GalleryClient, test_utils::*};

    #[tokio::test]
    async fn test_deserialize_gallery_album_remote() {
        let client = get_basic_client().unwrap();
        let res = client
            .get_gallery_album(&"HvCcoNA".into())
            .await
            .unwrap()
            .content
            .result()
            .unwrap();
        println!("{:#?}", res);
    }

    // TODO: Enable test once I get a correct response/good hash
    #[ignore = "Wrong implementation"]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() {
        let client = get_basic_client().unwrap();
        let res = client
            .get_gallery_image(&"MDCEW6Q".into())
            .await
            .unwrap()
            .content
            .result()
            .unwrap();
        println!("{:#?}", res);
    }
}
