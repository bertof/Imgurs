//! API wrapper

use std::error::Error;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::serialization::unix_epoch;

pub mod authorization;

#[derive(Clone, Debug)]
pub struct Client<'a> {
    pub(crate) inner: ReqwestClient,
    client_strings: ClientStrings<'a>,
}

impl<'a> Client<'a> {
    pub fn new(client_id: &'a str, client_secret: Option<&'a str>, tokens: Option<Tokens<'a>>) -> Result<Self, Box<dyn Error>> {
        let client = ReqwestClientBuilder::new()
            .default_headers(vec![
                (HeaderName::from_str("Authorization")?,
                 HeaderValue::from_str(&format!("Client-ID {}", client_id))?)
            ].iter().cloned().collect())
            .build()?;
        Ok(Client {
            inner: client,
            client_strings: ClientStrings {
                client_id,
                client_secret,
                tokens,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClientStrings<'a> {
    client_id: &'a str,
    client_secret: Option<&'a str>,
    tokens: Option<Tokens<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tokens<'a> {
    access_token: Option<&'a str>,
    refresh_token: Option<&'a str>,
    #[serde(with = "unix_epoch")]
    expires_in: DateTime<Utc>,
}

