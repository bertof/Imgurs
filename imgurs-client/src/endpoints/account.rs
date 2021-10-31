//! Account API implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use imgurs_model::model::{
    account::{Account, AccountBlocks, BlockedStatus},
    basic::{Basic, Data},
    common::Username,
    custom_gallery::CustomGalleryItem,
};

use crate::{
    client::{AuthenticatedClient, BasicClient, SortPreference},
    error::ClientError,
    response::Response,
    traits::{Client, RegisteredClient},
};

/// Account API client
#[async_trait]
pub trait AccountClient: Client {
    /// Get account information by username
    async fn get_account_by_username(
        &self,
        username: Username,
    ) -> Result<Response<Account>, ClientError> {
        let res = self
            .get_client()
            .get(&format!("https://api.imgur.com/3/account/{}", username))
            .headers(self.get_headers()?)
            .send()
            .await?;
        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    // /// Get account by user id
    // async fn get_account_by_id(&self, _user_id: AccountID) -> Result<serde_json::Value, ClientError> {
    //     unimplemented!()
    //
    //     // TODO: can't get how to implement it. Unclear API documentation.
    //     // self.get_client()
    //     //     .get(&format!("https://api.imgur.com/3/account/{}?user_id=True", user_id))
    //     //     .headers(self.get_headers()?)
    //     //     .send().await?
    //     //     .json::<serde_json::Value>().await
    //     //     .map_err(Into::into)
    // }

    /// Get account block status
    async fn get_account_block_status(
        &self,
        username: &str,
    ) -> Result<Response<BlockedStatus>, ClientError> {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[serde(deny_unknown_fields)]
        struct Wrapper {
            data: BlockedStatus,
        }

        let res = self
            .get_client()
            .get(&format!(
                "https://api.imgur.com/account/v1/{}/block",
                username
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let status = res.status();
        let data = Data::Content(res.json::<Wrapper>().await?.data);

        Ok(Response {
            content: Basic {
                data,
                success: status.is_success(),
                status: status.as_u16(),
            },
            headers,
        })
    }

    /// Get account images
    ///
    /// TODO: missing response typing
    async fn get_account_images(
        &self,
        username: &str,
    ) -> Result<Response<Vec<serde_json::Value>>, ClientError> {
        let res = self
            .get_client()
            .get(&format!(
                "https://api.imgur.com/3/account/{}/images",
                &username
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Account gallery favorites
    ///
    /// Return the images the user has favorited in the gallery
    async fn get_gallery_favorites(
        &self,
        username: &str,
        page: Option<u64>,
        sort: Option<SortPreference>,
    ) -> Result<Response<Vec<CustomGalleryItem>>, ClientError> {
        let mut url = format!(
            "https://api.imgur.com/3/account/{}/gallery_favorites",
            username
        );
        if let Some(page) = page {
            url = format!("{}/{}", url, page);
        }
        if let Some(sort) = sort {
            url = format!("{}/{}", url, sort);
        }

        println!("{}", url);

        let res = self
            .get_client()
            .get(&url)
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let text = res.text().await?;

        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&text)?)?
        );

        let content = serde_json::from_str(&text)?;
        // let content = res.json().await?;

        Ok(Response { content, headers })
    }
}

/// Registered client account API client
#[async_trait]
pub trait AccountRegisteredClient: AccountClient + RegisteredClient {
    /// Get list of blocked accounts
    async fn get_account_blocks(&self) -> Result<Basic<AccountBlocks>, ClientError> {
        self.get_client()
            .get("https://api.imgur.com/3/account/me/block")
            .headers(self.get_headers()?)
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create a block for an account
    async fn create_account_block(
        &self,
        username: &str,
    ) -> Result<Response<BlockedStatus>, ClientError> {
        let res = self
            .get_client()
            .put(&format!(
                "https://api.imgur.com/account/v1/{}/block",
                username
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Remove a block for an account
    async fn remove_account_block(
        &self,
        username: &str,
    ) -> Result<Response<BlockedStatus>, ClientError> {
        let res = self
            .get_client()
            .delete(&format!(
                "https://api.imgur.com/account/v1/{}/block",
                username
            ))
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Get images of the current user
    async fn get_user_account_images(
        &self,
    ) -> Result<Response<Vec<serde_json::Value>>, ClientError> {
        self.get_account_images("me").await
    }

    /// Get favourite galleries of the current user
    async fn get_user_gallery_favorites(
        &self,
        page: Option<u64>,
        sort: Option<SortPreference>,
    ) -> Result<Response<Vec<CustomGalleryItem>>, ClientError> {
        self.get_gallery_favorites("me", page, sort).await
    }

    // TODO: typed implementation
    /// Get favourites of the current user
    async fn get_user_favorites(
        &self,
        page: Option<u64>,
        sort: Option<SortPreference>,
    ) -> Result<Response<serde_json::Value>, ClientError> {
        let mut url = "https://api.imgur.com/3/account/me/favorites".to_string();
        if let Some(page) = page {
            url = format!("{}/{}", url, page);
        }
        if let Some(sort) = sort {
            url = format!("{}/{}", url, sort);
        }

        println!("{}", url);

        let res = self
            .get_client()
            .get(&url)
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        println!("{:?}", headers);
        let content = res.json().await?;

        Ok(Response { content, headers })
    }

    /// Account submissions
    ///
    /// Return the images a user has submitted to the gallery. You can add sorting as well after paging. Sorts can be: newest (default), oldest, worst, best.
    async fn get_user_submissions(
        &self,
        page: Option<u64>,
        sort: Option<SortPreference>,
    ) -> Result<Response<serde_json::Value>, ClientError> {
        let mut url = "https://api.imgur.com/3/account/me/submissions".to_string();
        if let Some(page) = page {
            url = format!("{}/{}", url, page);
        }
        if let Some(sort) = sort {
            url = format!("{}/{}", url, sort);
        }

        let res = self
            .get_client()
            .get(&url)
            .headers(self.get_headers()?)
            .send()
            .await?;

        let headers = res.headers().clone();
        let content = res.json().await?;

        Ok(Response { content, headers })
    }
}

impl AccountClient for BasicClient {}

impl AccountClient for AuthenticatedClient {}

impl AccountRegisteredClient for AuthenticatedClient {}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use chrono::Utc;

    use imgurs_model::{
        model::authorization::{AccessToken, ClientID, ClientSecret, RefreshToken},
        traits::from_env::FromEnv,
    };

    use crate::endpoints::account::AccountRegisteredClient;
    use crate::endpoints::authorization::AuthenticationRegisteredClient;
    use crate::{client::BasicClient, endpoints::account::AccountClient};

    #[tokio::test]
    async fn test_get_account_by_username() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client
            .get_account_by_username("bertof".into())
            .await?
            .content
            .result()?;

        println!("{:#?}", res);
        assert_eq!(&res.url.to_string(), "bertof");
        assert_eq!(res.id.to_string(), "57420253");

        Ok(())
    }

    // TODO: enable test once method is implemented
    /*#[tokio::test]
    async fn test_get_account_by_user_id() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client
            .get_account_by_id(57420253).await?;

        println!("{:#?}", res);

        Ok(())
    }*/

    #[tokio::test]
    async fn test_get_account_block_status() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?;
        let res = client.get_account_block_status(&"bertof".into()).await?;

        println!("{:#?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_blocks() -> Result<(), Box<dyn Error>> {
        // Sorry RansackThaElder, needed a test user ☺
        // let target_user = "RansackThaElder".to_string();

        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let access_token = AccessToken::from_default_env()??;
        let refresh_token = RefreshToken::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?
            .with_fresh_tokens()
            .await?;

        let res = client.get_account_blocks().await?.result()?;
        println!("{:#?}", res);
        assert!(res.items.is_empty());
        assert_eq!(res.next, None);

        // TODO: enable test once method is implemented
        // let res = client
        //     .create_account_block(&target_user)
        //     .await?;
        // println!("{:#?}", res);
        //
        // tokio::time::delay_for(Duration::from_secs(1)).await;
        //
        // let res = client
        //     .get_account_blocks()
        //     .await?;
        // println!("{:#?}", res);
        //
        // tokio::time::delay_for(Duration::from_secs(1)).await;
        //
        // let res = client
        //     .remove_account_block(&target_user)
        //     .await?;
        // println!("{:#?}", res);
        //
        // tokio::time::delay_for(Duration::from_secs(1)).await;
        //
        // let res = client
        //     .with_fresh_tokens()
        //     .get_account_blocks()
        //     .await?;
        // println!("{:#?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_account_images() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let access_token = AccessToken::from_default_env()??;
        let refresh_token = RefreshToken::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?
            .with_fresh_tokens()
            .await?;

        let res = client.get_user_account_images().await?.content.result()?;

        println!("{:#?}", res);

        Ok(())
    }

    // TODO: enable test once parsing is corrected
    #[ignore]
    #[tokio::test]
    async fn test_get_account_gallery_favorites() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let access_token = AccessToken::from_default_env()??;
        let refresh_token = RefreshToken::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?
            .with_fresh_tokens()
            .await?;

        let res = client
            .get_user_gallery_favorites(None, None)
            .await?
            .content
            .result()?;

        println!("{:#?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_account_favorites() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let access_token = AccessToken::from_default_env()??;
        let refresh_token = RefreshToken::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?
            .with_fresh_tokens()
            .await?;

        let res = client
            .get_user_favorites(Some(0), None)
            .await?
            .content
            .result()?;

        println!("{:#?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_account_submissions() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()??;
        let client_secret = ClientSecret::from_default_env()??;
        let access_token = AccessToken::from_default_env()??;
        let refresh_token = RefreshToken::from_default_env()??;
        let client = BasicClient::new(client_id, client_secret)?
            .with_tokens(access_token, refresh_token, Utc::now())?
            .with_fresh_tokens()
            .await?;

        let res = client
            .get_user_submissions(Some(0), None)
            .await?
            .content
            .result()?;

        println!("{:#?}", res);

        Ok(())
    }
}
