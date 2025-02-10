use crate::pds::noauth::PublicAtAgent;
use crate::pds::PdsDomain;
use atrium_api::com::atproto::sync::list_repos;

impl PublicAtAgent {
    pub async fn com_atproto_sync_listRepos(
        &self,
        pds_domain: PdsDomain,
        parameters: list_repos::ParametersData,
    ) -> Result<list_repos::OutputData, anyhow::Error> {
        let mut url = format!("https://{pds_domain}/xrpc/{}", list_repos::NSID);

        let params = serde_urlencoded::to_string(&parameters)?;
        url = format!("{url}?{params}");

        Ok(reqwest::get(url).await?.json().await?)
    }
}
