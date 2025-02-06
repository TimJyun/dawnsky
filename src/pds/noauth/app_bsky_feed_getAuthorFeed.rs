use crate::pds::noauth::PublicAtAgent;
use atrium_api::app::bsky::feed::get_author_feed;

impl PublicAtAgent {
    pub async fn app_bsky_feed_getAuthorFeed(
        &self,
        parameters: atrium_api::app::bsky::feed::get_author_feed::ParametersData,
    ) -> Result<atrium_api::app::bsky::feed::get_author_feed::Output, anyhow::Error> {
        let mut url = format!("https://public.api.bsky.app/xrpc/{}", get_author_feed::NSID);

        let params = serde_urlencoded::to_string(&parameters)?;
        url = format!("{url}?{params}");

        Ok(reqwest::get(url).await?.json().await?)
    }
}
