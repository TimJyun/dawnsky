use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::admin::delete_account;

impl PdsAdminAgent {
    pub async fn com_atproto_admin_deleteAccount(
        &self,
        parameter: delete_account::InputData,
    ) -> Result<(), anyhow::Error> {
        let mut output = reqwest::Client::new()
            .post(format!(
                "https://{}/xrpc/{}",
                self.pds_site,
                delete_account::NSID
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
