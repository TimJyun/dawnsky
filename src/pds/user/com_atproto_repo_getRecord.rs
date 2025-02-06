use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn com_atproto_repo_getRecord(
        &self,
        parameters: atrium_api::com::atproto::repo::get_record::ParametersData,
    ) -> Result<atrium_api::com::atproto::repo::get_record::Output, anyhow::Error> {
        Ok(self
            .send_get(
                atrium_api::com::atproto::repo::get_record::NSID,
                Some(parameters),
                None,
            )
            .await?)
    }
}
