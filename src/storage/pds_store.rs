use crate::pds::admin::PdsAdminAgent;
use std::sync::Arc;

pub async fn get_pds_admin_agent() -> Result<Arc<PdsAdminAgent>, anyhow::Error> {
    //todo : pds 管理支持
    // #[cfg(target_family = "wasm")]
    // {
    // }
    Err(anyhow::Error::msg("not store device"))
}
