use bsky_sdk::agent::config::Config;
use bsky_sdk::BskyAgent;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod agent;

pub mod admin;
pub mod noauth;
pub mod record;
pub mod store;
pub mod user;
pub mod user_did;
pub mod util;

pub static PUBLIC_AGENT: Lazy<BskyAgent> = Lazy::new(|| {
    futures::executor::block_on(
        BskyAgent::builder()
            .config(Config {
                endpoint: "https://public.api.bsky.app".to_string(),
                session: None,
                labelers_header: None,
                proxy_header: None,
            })
            .build(),
    )
    .expect("failed to build Bluesky Agent , it should not happen")
});
