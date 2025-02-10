use crate::error::CustomError;
use crate::pds::user::{AtAgent, Session};
use std::str::FromStr;

use crate::storage::session_store::get_session_store;
use atrium_api::com::atproto::server::create_account;
use atrium_api::types::string::{Did, Handle};
use opendal::raw::oio::Read;
use opendal::{Configurator, Operator};
use opendal_indexeddb::config::IndexeddbConfig;
use std::sync::Arc;
use tracing::error;

pub struct IndexeddbSessionStore {
    op: Operator,
}

impl IndexeddbSessionStore {
    pub async fn new(object_store_name: impl AsRef<str>) -> Result<Self, anyhow::Error> {
        let session_store_config: IndexeddbConfig = IndexeddbConfig {
            db_name: None,
            object_store_name: Some(object_store_name.as_ref().to_string()),
            root: None,
        };

        let builder = session_store_config.into_builder();
        let op = Operator::new(builder)?.finish();
        Ok(Self { op })
    }

    pub async fn default() -> Result<Self, anyhow::Error> {
        IndexeddbSessionStore::new("sessions").await
    }
}

impl SessionStore for IndexeddbSessionStore {
    async fn get_session(&self, did: Did) -> Result<Session, anyhow::Error> {
        let buff = self.op.read(did.as_str()).await?.to_vec();
        let session = ciborium::from_reader::<Session, _>(buff.as_slice())?;
        Ok(session)
    }

    async fn store_session(&self, session: &Session) -> Result<(), anyhow::Error> {
        let mut buff = Vec::<u8>::new();
        ciborium::into_writer(&session, &mut buff)?;
        self.op.write(session.did.as_str(), buff).await?;

        Ok(())
    }

    async fn remove_session(&self, did: Did) -> Result<(), anyhow::Error> {
        Ok(self.op.delete(did.as_str()).await?)
    }

    async fn list_sessions(&self) -> Result<Vec<Did>, anyhow::Error> {
        Ok(self
            .op
            .list("")
            .await?
            .into_iter()
            .filter_map(|e| match Did::new(e.path().to_string()) {
                Ok(did) => Some(did),
                Err(err) => {
                    error!("{}", err);
                    None
                }
            })
            .collect())
    }
}

pub trait SessionStore {
    async fn get_session(&self, did: Did) -> Result<Session, anyhow::Error>;
    async fn store_session(&self, session: &Session) -> Result<(), anyhow::Error>;
    async fn remove_session(&self, did: Did) -> Result<(), anyhow::Error>;
    async fn list_sessions(&self) -> Result<Vec<Did>, anyhow::Error>;
}

pub trait SessionStoreOperator {
    type Store: SessionStore;
    async fn create_session(
        &self,
        handle: String,
        password: String,
    ) -> Result<AtAgent<Self::Store>, CustomError>;

    async fn get_agent(&self, did: Did) -> Result<AtAgent<Self::Store>, CustomError>;
    async fn create_account(
        &self,
        handle: String,
        password: String,
        email: String,
        invite_code: Option<String>,
    ) -> Result<create_account::Output, anyhow::Error>;
}

impl<S: SessionStore> SessionStoreOperator for Arc<S> {
    type Store = S;
    async fn create_session(
        &self,
        handle: String,
        password: String,
    ) -> Result<AtAgent<Self::Store>, CustomError> {
        let endpoint = format!("https://{}", handle.as_str());
        let session = reqwest::Client::new()
            .post(format!("{endpoint}/xrpc/com.atproto.server.createSession"))
            .json(
                &bsky_sdk::api::com::atproto::server::create_session::InputData {
                    identifier: handle,
                    password,
                    allow_takendown: None,
                    auth_factor_token: None,
                },
            )
            .send()
            .await?
            .json::<Session>()
            .await?;

        let did = session.did.clone();
        self.store_session(&session).await?;

        Ok(AtAgent {
            session_store: self.clone(),
            did,
            session,
        })
    }

    async fn get_agent(&self, did: Did) -> Result<AtAgent<Self::Store>, CustomError> {
        let session = self.get_session(did.clone()).await?;

        Ok(AtAgent {
            session_store: self.clone(),
            did,
            session,
        })
    }
    async fn create_account(
        &self,
        handle: String,
        password: String,
        email: String,
        invite_code: Option<String>,
    ) -> Result<create_account::Output, anyhow::Error> {
        // let handle = format!("{username}.{pds}");
        let url = format!("https://{handle}/xrpc/com.atproto.server.createAccount");

        let output = reqwest::Client::new()
            .post(url)
            .json(&create_account::InputData {
                did: None,
                email: Some(email.clone()),
                handle: Handle::from_str(handle.as_str())
                    .map_err(|e| anyhow::Error::msg(format!("{e}:{handle}")))?,
                invite_code,
                password: Some(password),
                plc_op: None,
                recovery_key: None,
                verification_code: None,
                verification_phone: None,
            })
            .send()
            .await?
            .json::<create_account::Output>()
            .await?;

        self.store_session(&Session {
            access_jwt: output.access_jwt.clone(),
            active: None,
            did: output.did.clone(),
            did_doc: output.did_doc.clone(),
            email: Some(email),
            email_auth_factor: None,
            email_confirmed: None,
            handle: output.handle.clone(),
            refresh_jwt: output.refresh_jwt.clone(),
            status: None,
        })
        .await?;

        Ok(output)
    }
}
