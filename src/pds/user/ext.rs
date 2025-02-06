use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;
use atrium_api::app::bsky::actor::{profile, Profile};
use atrium_api::com::atproto::repo::{get_record, put_record};
use atrium_api::types::{Collection, TryFromUnknown, TryIntoUnknown};

impl<S: SessionStore> AtAgent<S> {
    pub async fn ext_get_my_profile(&self) -> Result<profile::RecordData, anyhow::Error> {
        let profile_record = self
            .com_atproto_repo_getRecord(get_record::ParametersData {
                cid: None,
                collection: Profile::nsid(),
                repo: self.did.clone().into(),
                rkey: "self".to_string(),
            })
            .await?;

        let profile = profile::RecordData::try_from_unknown(profile_record.value.to_owned())?;
        Ok(profile)
    }

    pub async fn ext_set_my_profile(
        &self,
        profile: profile::RecordData,
    ) -> Result<(), anyhow::Error> {
        self.com_atproto_repo_putRecord(put_record::InputData {
            collection: Profile::nsid(),
            record: profile.try_into_unknown()?,
            repo: self.did.clone().into(),
            rkey: "self".to_string(),
            swap_commit: None,
            swap_record: None,
            validate: None,
        })
        .await?;

        Ok(())
    }
}
