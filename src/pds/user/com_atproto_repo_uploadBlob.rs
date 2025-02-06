use crate::pds::store::SessionStore;
use crate::pds::user::AtAgent;

impl<S: SessionStore> AtAgent<S> {
    pub async fn com_atproto_repo_uploadBlob(
        &self,
        blob: Vec<u8>,
        content_type: impl AsRef<str>,
    ) -> Result<atrium_api::com::atproto::repo::upload_blob::Output, anyhow::Error> {
        use atrium_api::com::atproto::repo::upload_blob::NSID;

        let mut session = self.get_session().await?;

        let url = format!("https://{}/xrpc/{}", session.handle.as_str(), NSID);

        let mut request_builder = reqwest::Client::new().post(&url);

        request_builder = request_builder.body(blob);

        request_builder =
            Self::set_authorization(session.access_jwt.as_str(), request_builder).await?;

        request_builder =
            request_builder.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());

        Ok(request_builder.send().await?.json().await?)
    }
}
