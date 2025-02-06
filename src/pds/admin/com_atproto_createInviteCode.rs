use crate::pds::admin::PdsAdminAgent;
use atrium_api::com::atproto::server::create_invite_code;

use create_invite_code::InputData;
use create_invite_code::Output;
use create_invite_code::NSID;

impl PdsAdminAgent {
    pub async fn com_atproto_createInviteCode(
        &self,
        parameter: InputData,
    ) -> Result<Output, anyhow::Error> {
        self.post(NSID, &parameter).await
    }
}
