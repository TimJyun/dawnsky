use crate::pds::noauth::{Blob, PublicAtAgent, Url};
use atrium_api::com::atproto::sync::get_blob;

impl PublicAtAgent {
    pub fn com_atproto_sync_getBlob(
        &self,
        parameters: atrium_api::com::atproto::sync::get_blob::ParametersData,
    ) -> Result<Url<Blob>, anyhow::Error> {
        let mut url = format!("{}/xrpc/{}", self.endpoint.as_ref(), get_blob::NSID,);

        let params = serde_urlencoded::to_string(&parameters)?;
        url = format!("{url}?{params}");

        Ok(Url::new(url))
    }
}
