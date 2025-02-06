use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::admin::update_subject_status;

impl PdsAdminAgent {
    pub async fn com_atproto_admin_updateSubjectStatus(
        &self,
        parameter: update_subject_status::InputData,
    ) -> Result<update_subject_status::Output, anyhow::Error> {
        let mut output = reqwest::Client::new()
            .post(format!(
                "https://{}/xrpc/{}",
                self.pds_site,
                update_subject_status::NSID
            ))
            .basic_auth("admin", Some(self.token.as_str()))
            .json(&parameter)
            .send()
            .await?
            .json::<update_subject_status::Output>()
            .await?;

        Ok(output)
    }
}
