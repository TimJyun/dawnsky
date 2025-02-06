use crate::pds::store::IndexeddbSessionStore;
use std::sync::Arc;

use async_once_cell::OnceCell;

static GLOBAL_STORE: OnceCell<Arc<IndexeddbSessionStore>> = OnceCell::new();

pub async fn get_session_store() -> Result<Arc<IndexeddbSessionStore>, anyhow::Error> {
    #[cfg(target_family = "wasm")]
    {
        let session_store = GLOBAL_STORE
            .get_or_init(async {
                Arc::new(
                    IndexeddbSessionStore::default()
                        .await
                        .expect("init indexeddb session store failed"),
                )
            })
            .await;

        return Ok(session_store.clone());
    }
    Err(anyhow::Error::msg("not store device"))
}
