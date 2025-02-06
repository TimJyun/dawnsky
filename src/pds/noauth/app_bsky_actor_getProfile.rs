use crate::pds::noauth::PublicAtAgent;
use atrium_api::app::bsky::actor::get_profile;

impl PublicAtAgent {
    pub async fn app_bsky_actor_getProfile(
        &self,
        parameters: atrium_api::app::bsky::actor::get_profile::ParametersData,
    ) -> Result<atrium_api::app::bsky::actor::get_profile::Output, anyhow::Error> {
        let mut url = format!("https://public.api.bsky.app/xrpc/{}", get_profile::NSID);

        let params = serde_urlencoded::to_string(&parameters)?;
        url = format!("{url}?{params}");

        Ok(reqwest::get(url).await?.json().await?)
    }
}
