use atrium_api::types::string::{Did, Handle};
use dioxus::prelude::{Readable, Writable};
use std::borrow::Borrow;

use http::header::InvalidHeaderValue;

use crate::pds::store::{IndexeddbSessionStore, SessionStore, SessionStoreOperator};
use crate::pds::user::{AtAgent, Session};
use crate::storage::session_store::get_session_store;
use thiserror::Error;
use tracing::error;

pub const MAIN_SESSION_DID: &'static str = "main-session-did";

#[derive(Error, Debug)]
pub enum GetAgentError {
    #[error("InvalidHeaderValue:{0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("ReqwestError:{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("BskySdkError:{0}")]
    BskySdkError(#[from] bsky_sdk::Error),
}

pub fn get_main_session_did() -> Result<Did, anyhow::Error> {
    #[cfg(target_family = "wasm")]
    {
        use gloo_storage::{LocalStorage, Storage};
        let main_did = LocalStorage::get::<Did>(crate::pds::agent::MAIN_SESSION_DID)?;
        return Ok(main_did);
    }

    Err(anyhow::Error::msg("it is not wasm env"))
}

pub async fn get_main_session() -> Result<Session, anyhow::Error> {
    #[cfg(target_family = "wasm")]
    {
        use gloo_storage::{LocalStorage, Storage};

        let main_did = LocalStorage::get::<Did>(crate::pds::agent::MAIN_SESSION_DID)?;

        let session_store = crate::storage::session_store::get_session_store().await?;

        let session = session_store.get_session(main_did).await?;
        return Ok(session);
    }

    Err(anyhow::Error::msg("it is not wasm env"))
}

pub async fn get_main_agent() -> Result<AtAgent<IndexeddbSessionStore>, anyhow::Error> {
    #[cfg(target_family = "wasm")]
    {
        use gloo_storage::{LocalStorage, Storage};
        let session_store = get_session_store().await?;

        let main_did = LocalStorage::get::<Did>(crate::pds::agent::MAIN_SESSION_DID)?;

        let agent = session_store.get_agent(main_did).await?;

        return Ok(agent);
    }

    Err(anyhow::Error::msg("it is not wasm env"))
}

pub async fn login(handle: Handle, password: String) -> Result<(), anyhow::Error> {
    #[cfg(target_family = "wasm")]
    {
        use gloo_storage::LocalStorage;
        use gloo_storage::Storage;
        let session_store = get_session_store().await?;

        let at_agent = session_store
            .create_session(handle.as_str().to_string(), password)
            .await?;

        let session = at_agent.get_session().await?;

        let _ = LocalStorage::set(crate::pds::agent::MAIN_SESSION_DID, &session.did);

        return Ok(());
    }

    Err(anyhow::Error::msg("it is not wasm env"))
}
