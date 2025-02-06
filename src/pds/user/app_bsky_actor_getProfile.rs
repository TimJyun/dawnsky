use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn app_bsky_actor_getProfile(
        &self,
        parameters: atrium_api::app::bsky::actor::get_profile::ParametersData,
    ) -> Result<atrium_api::app::bsky::actor::get_profile::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::app::bsky::actor::get_profile::NSID,
                Some(parameters),
                None,
            )
            .await?)
    }
}
