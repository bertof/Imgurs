//! Gallery API implementation

use async_trait::async_trait;

use crate::{
    api::{
        client::{AuthenticatedClient, BasicClient},
        error::ClientError,
        response::Response,
        traits::{Client, RegisteredClient},
    },
    model::{
        album::AlbumID,
        gallery_album::GalleryAlbum,
        gallery_image::{GalleryImage, GalleryImageID},
    },
    serialization::pretty_json,
};

/// Gallery API client
#[async_trait]
pub trait GalleryClient: Client {
    /// Gallery album
    ///
    /// Get additional information about an album in the gallery.
    async fn get_gallery_album(&self, album_id: &AlbumID) -> Result<Response<GalleryAlbum>, ClientError> {
        let res = self.get_client()
            .get(&format!("https://api.imgur.com/3/gallery/album/{}", album_id))
            .headers(self.get_headers()?)
            .send().await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response {
            content,
            headers,
        })
    }

    /// Gallery image
    ///
    /// Get additional information about an image in the gallery.
    async fn get_gallery_image(&self, gallery_image_id: &GalleryImageID) -> Result<Response<GalleryImage>, ClientError> {
        let res = self.get_client()
            .get(&format!("https://api.imgur.com/3/gallery/image/{id}", id = gallery_image_id))
            .headers(self.get_headers()?)
            .send().await?;

        let headers = res.headers().clone();
        println!("{:#?}", headers);

        let text = res.text().await?;
        println!("{}", text);
        println!("{}", pretty_json(&text)?);

        let content = serde_json::from_str(&text)?;
        // let content = res.json().await?;

        Ok(Response {
            content,
            headers,
        })
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
    use std::error::Error;

    use crate::{
        api::{
            client::BasicClient,
            endpoints::gallery::GalleryClient,
        },
        model::authorization::{ClientID, ClientSecret},
        traits::FromEnv,
    };

    #[tokio::test]
    async fn test_deserialize_gallery_album_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = BasicClient::new(client_id, client_secret)?;

        let res = client
            .get_gallery_album(&"HvCcoNA".to_string()).await?
            .content.result()?;

        println!("{:#?}", res);

        Ok(())
    }

    // TODO: Enable test once I get a correct response/good hash
    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = BasicClient::new(client_id, client_secret)?;

        let res = client
            .get_gallery_image(&"MDCEW6Q".to_string()).await?
            .content.result()?;

        println!("{:#?}", res);

        Ok(())
    }
}