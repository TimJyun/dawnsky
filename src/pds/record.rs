use atrium_api::types::Collection;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Record: Serialize + DeserializeOwned {
    const NSID: &'static str;
}

impl Record for atrium_api::app::bsky::feed::post::RecordData {
    const NSID: &'static str = atrium_api::app::bsky::feed::Post::NSID;
}
