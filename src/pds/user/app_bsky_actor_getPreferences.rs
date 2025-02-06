use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn app_bsky_actor_getPreferences(
        &self,
        parameters: atrium_api::app::bsky::actor::get_preferences::ParametersData,
    ) -> Result<atrium_api::app::bsky::actor::get_preferences::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::app::bsky::actor::get_preferences::NSID,
                Some(parameters),
                None,
            )
            .await?)
    }
}
