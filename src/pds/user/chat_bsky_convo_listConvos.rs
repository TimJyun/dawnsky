use crate::pds::store::SessionStore;
use crate::pds::user::{AtAgent, SERVICE__BSKY_CHAT};

impl<S: SessionStore> AtAgent<S> {
    pub async fn chat_bsky_convo_listConvos(
        &self,
        parameters: atrium_api::chat::bsky::convo::list_convos::ParametersData,
    ) -> Result<atrium_api::chat::bsky::convo::list_convos::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::chat::bsky::convo::list_convos::NSID,
                Some(parameters),
                Some(SERVICE__BSKY_CHAT),
            )
            .await?)
    }
}
