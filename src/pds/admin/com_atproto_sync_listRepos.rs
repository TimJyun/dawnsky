use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::sync::list_repos;

use list_repos::Output;
use list_repos::ParametersData;
use list_repos::NSID;

impl PdsAdminAgent {
    pub async fn com_atproto_sync_listRepos(
        &self,
        parameter: ParametersData,
    ) -> Result<Output, anyhow::Error> {
        self.get_with_parameter(NSID, &parameter).await
    }
}
