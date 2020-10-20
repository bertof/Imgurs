#[cfg(test)]
mod tests {
    use std::error::Error;
    use reqwest::{StatusCode, Method};

    #[tokio::test]
    async fn test_deserialize_account_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = BasicClient::new(client_id, client_secret)?;

        let account = client.get_client()
            .get("https://api.imgur.com/3/account/ghostinspector")
            .send().await?
            .json::<Basic<Account>>().await?;

        println!("{:#?}", account);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_album_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = BasicClient::new(client_id, client_secret)?;

        let data = client.get_client()
            .get("https://api.imgur.com/3/album/z6B0j")
            .send().await?
            .json::<Basic<Album>>().await?;

        println!("{:?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_deserialize_comment_remote() -> Result<(), Box<dyn Error>> {
        let client_id = ClientID::from_default_env()?;
        let client_secret = ClientSecret::from_default_env()?;
        let client = BasicClient::new(client_id, client_secret)?;

        let data = client.get_client()
            .get("https://api.imgur.com/3/comment/1938633683")
            .send().await?
            .json::<Basic<Comment>>().await?;

        println!("{:?}", data);

        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_account_settings_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_conversation_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_custom_gallery_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_profile_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[ignore]
    #[tokio::test]
    async fn test_deserialize_gallery_image_remote() -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    #[tokio::test]
    async fn test_error_parsing_remote() -> Result<(), Box<dyn Error>> {
        let res = reqwest::get("https://api.imgur.com/3/account/me/settings").await?;
        assert_eq!(res.status(), StatusCode::from_u16(401)?);

        let data = res.json::<Basic<AccountSettings>>().await?;
        assert!(!data.success);
        assert_eq!(data.status, 401);
        match data.data {
            Data::Content(_) => panic!("Should return error"),
            Data::Error { error, request, method } => {
                assert_eq!(error, ErrorMessage::new("Authentication required"));
                assert_eq!(request, "/3/account/me/settings");
                assert_eq!(method, Method::GET);
            }
        }

        Ok(())
    }
}