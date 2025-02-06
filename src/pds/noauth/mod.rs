use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Error;
use std::borrow::Cow;
use std::marker::PhantomData;
pub mod app_bsky_actor_getProfile;
mod app_bsky_feed_getAuthorFeed;
mod app_bsky_feed_getPostThread;
pub mod com_atproto_sync_getBlob;

pub struct PublicAtAgent {
    pub endpoint: Cow<'static, str>,
}

impl PublicAtAgent {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint: Cow::Owned(endpoint),
        }
    }

    pub fn from_handle(handle: impl AsRef<str>) -> Self {
        Self {
            endpoint: Cow::Owned(format!("https://{}", handle.as_ref())),
        }
    }
}
impl Default for PublicAtAgent {
    fn default() -> Self {
        Self {
            endpoint: Cow::Borrowed("https://public.api.bsky.app"),
        }
    }
}

pub struct Url<O: OutputFormat>(pub String, PhantomData<O>);

impl<O: OutputFormat> Url<O> {
    pub(crate) fn new(url: String) -> Url<O> {
        Url(url, Default::default())
    }
}

pub trait OutputFormat {
    type Output;

    fn des(blob: Vec<u8>) -> Result<Self::Output, serde_json::Error>
    where
        Self: Sized;
}

#[derive(Debug, Deserialize)]
pub struct Json<T>(PhantomData<T>);

impl<T: DeserializeOwned> OutputFormat for Json<T> {
    type Output = T;

    fn des(blob: Vec<u8>) -> Result<Self::Output, serde_json::Error> {
        Ok(serde_json::from_slice(blob.as_slice())?)
    }
}

pub struct Blob;

impl OutputFormat for Blob {
    type Output = Vec<u8>;

    fn des(blob: Vec<u8>) -> Result<Self::Output, Error> {
        Ok(blob)
    }
}

impl<O: OutputFormat> Url<O> {
    pub async fn get(self) -> Result<O::Output, anyhow::Error> {
        let bytes = reqwest::Client::new()
            .get(self.0)
            .send()
            .await?
            .bytes()
            .await?;

        Ok(O::des(bytes.to_vec())?)
    }
}
