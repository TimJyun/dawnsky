use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::admin::disable_invite_codes;

impl PdsAdminAgent {
    pub async fn com_atproto_admin_disableInviteCodes(
        &self,
        parameter: disable_invite_codes::InputData,
    ) -> Result<(), anyhow::Error> {
        let mut output = reqwest::Client::new()
            .post(format!(
                "https://{}/xrpc/{}",
                self.pds_site,
                disable_invite_codes::NSID
            ))
            .basic_auth("admin", Some(self.token.as_str()))
            .json(&parameter)
            .send()
            .await?
            .bytes()
            .await?;

        Ok(())
    }
}
