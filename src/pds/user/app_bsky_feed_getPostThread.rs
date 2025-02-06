use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn app_bsky_feed_getPostThread(
        &self,
        parameters: atrium_api::app::bsky::feed::get_post_thread::ParametersData,
    ) -> Result<atrium_api::app::bsky::feed::get_post_thread::OutputData, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::app::bsky::feed::get_post_thread::NSID,
                Some(parameters),
                None,
            )
            .await?)
    }
}
