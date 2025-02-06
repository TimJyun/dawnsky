use crate::pds::record::Record;
use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;
use atrium_api::com::atproto::repo::create_record;
use atrium_api::types::string::{AtIdentifier, Cid, Nsid};
use std::str::FromStr;

//from atrium_api::com::atproto::repo::create_record::InputData
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input<R> {
    pub collection: Nsid,
    pub record: R,
    pub repo: AtIdentifier,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub rkey: Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub swap_commit: Option<Cid>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub validate: Option<bool>,
}

impl<S: SessionStore> AtAgent<S> {
    pub async fn com_atproto_repo_createRecord<R: Record>(
        &self,
        record: R,
    ) -> Result<atrium_api::com::atproto::repo::create_record::OutputData, anyhow::Error> {
        let mut session = self.get_session().await?;
        let mut url = format!(
            "https://{}/xrpc/{}",
            session.handle.as_str(),
            create_record::NSID
        );
        let mut request_builder = reqwest::Client::new().post(&url);

        let json = Input {
            collection: Nsid::from_str(R::NSID).unwrap(),
            record,
            repo: session.did.clone().into(),
            rkey: None,
            swap_commit: None,
            validate: None,
        };

        request_builder = request_builder.json(&json);

        request_builder =
            Self::set_authorization(session.access_jwt.as_str(), request_builder).await?;

        Ok(request_builder.send().await?.json().await?)
    }
}
