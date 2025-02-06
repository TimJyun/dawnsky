mod app_bsky_actor_getPreferences;
mod app_bsky_actor_getProfile;
mod app_bsky_feed_getPostThread;
mod app_bsky_feed_getTimeline;
pub mod chat_bsky_convo_getConvo;
mod chat_bsky_convo_getMessages;
mod chat_bsky_convo_listConvos;
mod chat_bsky_convo_sendMessage;
mod com_atproto_repo_createRecord;
mod com_atproto_repo_deleteRecord;
mod com_atproto_repo_getRecord;
mod com_atproto_repo_putRecord;
mod com_atproto_repo_uploadBlob;
mod ext;

use crate::util::jwt::is_jwt_expired;
use std::str::FromStr;

use crate::pds::noauth::PublicAtAgent;
use crate::pds::store::SessionStore;
use atrium_api::com::atproto;
use atrium_api::types::string::Did;
use derive_more::Display;
use http::header::InvalidHeaderValue;
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

pub type Session = bsky_sdk::api::com::atproto::server::create_session::OutputData;

pub struct AtAgent<S: SessionStore> {
    pub session_store: Arc<S>,
    pub did: Did,
    pub session: Session,
}

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("serde json error : {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("request error : {0}")]
    RequestError(#[from] AgentRequestError),
    #[error("reqwest error : {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serde urlencoded error : {0}")]
    SerdeUrlencoded(#[from] serde_urlencoded::ser::Error),

    #[error("invalid token error")]
    InvalidJwt(Option<Did>),
    #[error("anyhow error : {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("invalid header value error : {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
}

#[derive(Serialize, Deserialize, Debug, Error, Display)]
#[display("code:{code},error:{error},message:{message}")]
pub struct AgentRequestError {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Method {
    Get,
    Post,
}

pub enum XrpcRequestData<D: Serialize> {
    Parameters(D),
    Input(D),
}

pub struct XrpcRequest<D: Serialize> {
    pub method: Method,
    pub nsid: &'static str,
    pub data: Option<XrpcRequestData<D>>,
}

impl<S: SessionStore> AtAgent<S> {
    pub async fn get_session(&self) -> Result<Arc<Session>, AgentError> {
        let mut session = self.session_store.get_session(self.did.clone()).await?;
        if is_jwt_expired(session.access_jwt.as_str()) {
            let mut old_session = Session::clone(&session);
            let authorization =
                HeaderValue::from_str(&format!("Bearer {}", old_session.refresh_jwt.as_str()))?;

            if let Ok(response) = reqwest::Client::new()
                .post(format!(
                    "https://{}/xrpc/com.atproto.server.refreshSession",
                    session.handle.as_str()
                ))
                .headers(HeaderMap::from_iter([(
                    HeaderName::from_static("authorization"),
                    authorization,
                )]))
                .send()
                .await
            {
                let response_status = response.status();
                let body = response.json::<serde_json::Value>().await?;
                if response_status == StatusCode::UNAUTHORIZED
                    || (response_status == StatusCode::BAD_REQUEST && {
                        let err_msg = body
                            .get("error")
                            .map(|e| e.to_string())
                            .ok_or_else(|| anyhow::Error::msg("deserialize fail"))?;
                        match err_msg.as_str() {
                            "ExpiredToken" | "InvalidToken" | "AccountTakedown" => true,
                            _ => {
                                return Err(anyhow::Error::msg(err_msg).into());
                            }
                        }
                    })
                {
                } else {
                    let new_session_data = serde_json::from_value::<
                        atproto::server::refresh_session::OutputData,
                    >(body)?;
                    let mut new_session = old_session;
                    new_session.access_jwt = new_session_data.access_jwt;
                    new_session.active = new_session_data.active;
                    new_session.did = new_session_data.did;
                    new_session.did_doc = new_session_data.did_doc;
                    new_session.handle = new_session_data.handle;
                    new_session.refresh_jwt = new_session_data.refresh_jwt;
                    new_session.status = new_session_data.status;

                    self.session_store.store_session(&new_session).await?;
                    session = new_session;
                }
            }
        }

        Ok(Arc::new(session))
    }

    pub(crate) async fn send_get<P: Serialize, O: DeserializeOwned>(
        &self,
        nsid: &str,
        params: Option<P>,
        proxy_header: Option<ProxyHeader>,
    ) -> Result<O, AgentError> {
        let mut session = self.get_session().await?;
        let is_official_pds = session.handle.ends_with(".bsky.social");
        let mut url = if let (Some(proxy_header), true) = (proxy_header, is_official_pds) {
            format!("https://{}/xrpc/{}", proxy_header.host, nsid,)
        } else {
            format!("https://{}/xrpc/{}", session.handle.as_str(), nsid,)
        };

        if let Some(params) = params.as_ref() {
            let params = serde_urlencoded::to_string(&params)?;
            url = format!("{url}?{params}");
        }

        let mut request_builder = reqwest::Client::new().get(&url);
        if let (Some(proxy_header), false) = (proxy_header, is_official_pds) {
            request_builder = proxy_header.set_proxy(request_builder);
        }

        request_builder =
            Self::set_authorization(session.access_jwt.as_str(), request_builder).await?;

        Ok(request_builder.send().await?.json().await?)
    }

    pub(crate) async fn send_post<I: Serialize, O: DeserializeOwned>(
        &self,
        nsid: &str,
        input: Option<I>,
        proxy_header: Option<ProxyHeader>,
    ) -> Result<O, AgentError> {
        let mut session = self.get_session().await?;
        let is_official_pds = session.handle.ends_with(".bsky.social");
        let mut url = if let (Some(proxy_header), true) = (proxy_header, is_official_pds) {
            format!("https://{}/xrpc/{}", proxy_header.host, nsid,)
        } else {
            format!("https://{}/xrpc/{}", session.handle.as_str(), nsid,)
        };
        let mut request_builder = reqwest::Client::new().post(&url);
        if let Some(input) = input.as_ref() {
            request_builder = request_builder.json(input);
        }
        if let (Some(proxy_header), false) = (proxy_header, is_official_pds) {
            request_builder = proxy_header.set_proxy(request_builder);
        }
        request_builder =
            Self::set_authorization(session.access_jwt.as_str(), request_builder).await?;
        Ok(request_builder.send().await?.json().await?)
    }

    async fn set_authorization(
        jwt: impl AsRef<str>,
        request_builder: RequestBuilder,
    ) -> Result<RequestBuilder, AgentError> {
        let authorization = HeaderValue::from_str(&format!("Bearer {}", jwt.as_ref()))?;

        Ok(request_builder.header(HeaderName::from_static("authorization"), authorization))
    }

    pub fn to_noath_agent(&self) -> PublicAtAgent {
        PublicAtAgent::from_handle(self.session.handle.as_str())
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ProxyHeader {
    pub(crate) host: &'static str,
    pub(crate) service_type: &'static str,
}

pub(crate) struct WebDid(&'static str);

pub const SERVICE__BSKY_CHAT: ProxyHeader = ProxyHeader {
    host: "api.bsky.chat",
    service_type: "bsky_chat",
};

impl ProxyHeader {
    fn set_proxy(self, request_builder: RequestBuilder) -> RequestBuilder {
        request_builder.header(
            HeaderName::from_static("atproto-proxy"),
            format!("did:web:{}#{}", self.host, self.service_type),
        )
    }
}
