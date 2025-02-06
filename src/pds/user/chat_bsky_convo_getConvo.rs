use crate::pds::store::SessionStore;
use crate::pds::user::{AtAgent, SERVICE__BSKY_CHAT};

impl<S: SessionStore> AtAgent<S> {
    pub async fn chat_bsky_convo_getConvo(
        &self,
        parameters: atrium_api::chat::bsky::convo::get_convo::ParametersData,
    ) -> Result<atrium_api::chat::bsky::convo::get_convo::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::chat::bsky::convo::get_convo::NSID,
                Some(parameters),
                Some(SERVICE__BSKY_CHAT),
            )
            .await?)
    }
}
