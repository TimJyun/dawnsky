use atrium_api::app::bsky::feed::defs::{ThreadViewPostData, ThreadViewPostParentRefs};
use atrium_api::com::atproto::repo::strong_ref;
use atrium_api::types::Union;

pub(crate) fn get_root(post: &ThreadViewPostData) -> strong_ref::MainData {
    if let Some(Union::Refs(ThreadViewPostParentRefs::ThreadViewPost(parent))) = &post.parent {
        get_root(parent)
    } else {
        strong_ref::MainData {
            cid: post.post.cid.clone(),
            uri: post.post.uri.clone(),
        }
    }
}
