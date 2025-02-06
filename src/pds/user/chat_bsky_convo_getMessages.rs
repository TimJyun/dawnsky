use crate::pds::store::SessionStore;
use crate::pds::user::{AtAgent, SERVICE__BSKY_CHAT};

impl<S: SessionStore> AtAgent<S> {
    pub async fn chat_bsky_convo_getMessages(
        &self,
        parameters: atrium_api::chat::bsky::convo::get_messages::ParametersData,
    ) -> Result<atrium_api::chat::bsky::convo::get_messages::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::chat::bsky::convo::get_messages::NSID,
                Some(parameters),
                Some(SERVICE__BSKY_CHAT),
            )
            .await?)
    }
}
