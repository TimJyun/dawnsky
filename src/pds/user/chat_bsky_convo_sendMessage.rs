use crate::pds::store::SessionStore;
use crate::pds::user::{AtAgent, SERVICE__BSKY_CHAT};

impl<S: SessionStore> AtAgent<S> {
    pub async fn chat_bsky_convo_sendMessageuse(
        &self,
        input: atrium_api::chat::bsky::convo::send_message::InputData,
    ) -> Result<atrium_api::chat::bsky::convo::send_message::Output, anyhow::Error> {
        Ok(self
            .send_post(
                atrium_api::chat::bsky::convo::send_message::NSID,
                Some(input),
                Some(SERVICE__BSKY_CHAT),
            )
            .await?)
    }
}
