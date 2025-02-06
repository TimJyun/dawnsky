use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;

pub mod com_atproto_admin_deleteAccount;
pub mod com_atproto_admin_disableInviteCodes;
pub mod com_atproto_admin_getInviteCodes;
pub mod com_atproto_admin_update_subject_status;
pub mod com_atproto_createInviteCode;
pub mod com_atproto_sync_listRepos;

pub struct PdsAdminAgent {
    pub pds_site: String,
    pub token: String,
}

impl PdsAdminAgent {
    pub(crate) async fn get<O: DeserializeOwned>(&self, NSID: &str) -> Result<O, anyhow::Error> {
        let url = format!("https://{}/xrpc/{}", self.pds_site, NSID);
        Ok(reqwest::Client::new()
            .get(url.as_str())
            .basic_auth("admin", Some(self.token.as_str()))
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub(crate) async fn get_with_parameter<P: Serialize, O: DeserializeOwned>(
        &self,
        NSID: &str,
        parameter: P,
    ) -> Result<O, anyhow::Error> {
        let params = serde_urlencoded::to_string(parameter)?;
        let url = format!("https://{}/xrpc/{}?{params}", self.pds_site, NSID);

        Ok(reqwest::Client::new()
            .get(url.as_str())
            .basic_auth("admin", Some(self.token.as_str()))
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub(crate) async fn post<I: Serialize, O: DeserializeOwned>(
        &self,
        NSID: &str,
        input: I,
    ) -> Result<O, anyhow::Error> {
        let url = format!("https://{}/xrpc/{}", self.pds_site, NSID);

        Ok(reqwest::Client::new()
            .get(url.as_str())
            .basic_auth("admin", Some(self.token.as_str()))
            .json(&input)
            .send()
            .await?
            .json::<O>()
            .await?)
    }
}
