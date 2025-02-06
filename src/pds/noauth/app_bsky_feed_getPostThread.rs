use crate::pds::noauth::PublicAtAgent;
use atrium_api::app::bsky::feed::get_post_thread;

impl PublicAtAgent {
    pub async fn app_bsky_feed_getPostThread(
        &self,
        parameters: atrium_api::app::bsky::feed::get_post_thread::ParametersData,
    ) -> Result<atrium_api::app::bsky::feed::get_post_thread::OutputData, anyhow::Error> {
        let mut url = format!("https://public.api.bsky.app/xrpc/{}", get_post_thread::NSID);

        let params = serde_urlencoded::to_string(&parameters)?;
        url = format!("{url}?{params}");

        Ok(reqwest::get(url).await?.json().await?)
    }
}
