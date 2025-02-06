use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn app_bsky_feed_getTimeline(
        &self,
        parameters: atrium_api::app::bsky::feed::get_timeline::ParametersData,
    ) -> Result<atrium_api::app::bsky::feed::get_timeline::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::app::bsky::feed::get_timeline::NSID,
                Some(parameters),
                None,
            )
            .await?)
    }
}
