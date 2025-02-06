use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::admin::get_invite_codes;

impl PdsAdminAgent {
    pub async fn com_atproto_admin_getInviteCodes(
        &self,
        parameter: get_invite_codes::ParametersData,
    ) -> Result<get_invite_codes::Output, anyhow::Error> {
        let params = serde_urlencoded::to_string(parameter)?;

        let url = if params.trim().is_empty() {
            format!("https://{}/xrpc/{}", self.pds_site, get_invite_codes::NSID)
        } else {
            format!(
                "https://{}/xrpc/{}?{params}",
                self.pds_site,
                get_invite_codes::NSID
            )
        };

        let mut output = reqwest::Client::new()
            .get(url.as_str())
            .basic_auth("admin", Some(self.token.as_str()))
            .send()
            .await?
            .json::<get_invite_codes::Output>()
            .await?;

        Ok(output)
    }
}
