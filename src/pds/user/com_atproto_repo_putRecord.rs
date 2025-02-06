use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn com_atproto_repo_putRecord(
        &self,
        input: atrium_api::com::atproto::repo::put_record::InputData,
    ) -> Result<atrium_api::com::atproto::repo::put_record::Output, anyhow::Error> {
        Ok(self
            .send_post(
                atrium_api::com::atproto::repo::put_record::NSID,
                Some(input),
                None,
            )
            .await?)
    }
}
