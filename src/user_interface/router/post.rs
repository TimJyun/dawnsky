use dioxus::prelude::*;

use crate::pds::agent::get_main_agent;

use crate::pds::PUBLIC_AGENT;
use crate::user_interface::app::refresh_app;
use crate::user_interface::component::loading::Loading;
use crate::user_interface::component::post_view::PostView;
use crate::user_interface::component::user_posts::UserPosts;
use atrium_api::app::bsky::actor::defs::{ProfileViewDetailed, ProfileViewDetailedData};
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::app::bsky::actor::get_profile::ParametersData;
use atrium_api::app::bsky::feed::get_post_thread::OutputThreadRefs;
use atrium_api::app::bsky::feed::{get_post_thread, get_quotes, post};
use atrium_api::com::atproto::identity::resolve_handle;
use atrium_api::types::string::{AtIdentifier, Datetime, Handle};
use atrium_api::types::Union;
use bsky_sdk::BskyAgent;
use chrono::{DateTime, FixedOffset};
use ipld_core::ipld::Ipld;

use crate::pds::noauth::PublicAtAgent;
use crate::pds::store::SessionStore;
use crate::pds::util::get_root;
use crate::storage::session_store::get_session_store;
use crate::user_interface::app::reload_page;
use crate::user_interface::component::select_account::{Direction, SelectAccount};
use anyhow::anyhow;
use atrium_api::app::bsky::feed::defs::{ThreadViewPostParentRefs, ThreadViewPostRepliesItem};
use atrium_api::app::bsky::feed::post::ReplyRefData;
use atrium_api::com::atproto::repo::{create_record, strong_ref};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::thread::Scope;
use tracing::debug;

#[component]
pub fn PostPage(user_handle: Handle, record_key: String) -> Element {
    let post_thread_res = use_resource({
        to_owned![user_handle, record_key];
        use_reactive!(|(user_handle, record_key)| {
            get_post_thread(user_handle.to_owned(), record_key.to_owned())
        })
    });

    let reply_data = use_resource(move || async move {
        let session_store = get_session_store().await.ok()?;
        let sessions = session_store.list_sessions().await.ok()?;
        if sessions.len() == 0 {
            return None;
        }

        let post_thread_read = post_thread_res.read();
        let post_thread_binding = post_thread_read.as_ref()?;
        let post_thread = post_thread_binding.as_ref().ok()?;
        if let Union::Refs(OutputThreadRefs::AppBskyFeedDefsThreadViewPost(post)) =
            &post_thread.thread
        {
            return Some(ReplyRefData {
                parent: strong_ref::MainData {
                    cid: post.post.cid.clone(),
                    uri: post.post.uri.clone(),
                }
                .into(),
                root: get_root(post).into(),
            });
        }
        None
    })
    .suspend()?;

    let selected = use_signal(|| None);
    let mut text = use_signal(String::new);

    let post_thread_read = post_thread_res.read();
    if let Some(post_thread) = post_thread_read.as_ref() {
        let post_view = match post_thread {
            Ok(post_thread) => {
                let p = if let Union::Refs(thread) = &post_thread.thread {
                    match thread {
                        OutputThreadRefs::AppBskyFeedDefsThreadViewPost(thread) => {
                            let reply_nodes = if let Some(replies) = thread.data.replies.as_ref() {
                                let reply_nodes = replies
                                    .into_iter()
                                    .filter_map(|reply| {
                                        if let Union::Refs(reply) = reply {
                                            Some(reply)
                                        } else {
                                            None
                                        }
                                    })
                                    .filter_map(|thread| {
                                        if let ThreadViewPostRepliesItem::ThreadViewPost(thread) =
                                            thread
                                        {
                                            Some(thread)
                                        } else {
                                            None
                                        }
                                    })
                                    .map(|thread| {
                                        rsx! {
                                            PostView { post_view: thread.post.data.clone() }
                                        }
                                    });

                                rsx! {
                                    {reply_nodes}
                                }
                            } else {
                                rsx! {}
                            };

                            rsx! {
                                PostView { post_view: thread.post.data.to_owned() }
                                {reply_nodes}
                            }
                        }
                        OutputThreadRefs::AppBskyFeedDefsNotFoundPost(not_found) => {
                            rsx! { "not_found" }
                        }
                        OutputThreadRefs::AppBskyFeedDefsBlockedPost(blocked) => {
                            rsx! { "blocked" }
                        }
                    }
                } else {
                    rsx! {}
                };
                rsx! {
                    {p}
                }
            }
            Err(err) => {
                rsx! {
                    div { "load error : {err}" }
                }
            }
        };

        if let Some(reply_data) = reply_data.read().as_ref().cloned() {
            rsx! {
                div { class: "w-full h-full flex flex-col",
                    div { class: "flex-1 overflow-y-scroll w-full", {post_view} }
                    div { class: "w-full",
                        div { class: "flex flex-row",
                            SelectAccount { selected, direction: Direction::Up }
                            textarea {
                                class: "flex-1 border min-h-[4.5rem] resize-none",
                                onchange: move |evt| {
                                    *text.write() = evt.value();
                                },
                            }
                            input {
                                r#type: "button",
                                value: "post",
                                onclick: move |_| {
                                    to_owned![reply_data];
                                    async move {
                                        match post_reply(reply_data, text.peek().to_string()).await {
                                            Ok(_) => {
                                                debug!("post_reply success");
                                                refresh_app();
                                            }
                                            Err(err) => {
                                                debug!("post_reply fail : {err}");
                                            }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        } else {
            rsx! {
                {post_view}
            }
        }
    } else {
        rsx! {
            Loading {}
        }
    }
}

async fn post_reply(
    reply_data: ReplyRefData,
    message: String,
) -> Result<create_record::OutputData, anyhow::Error> {
    let agent = get_main_agent().await?;
    let result = agent
        .com_atproto_repo_createRecord(post::RecordData {
            created_at: Datetime::now(),
            embed: None,
            entities: None,
            facets: None,
            labels: None,
            langs: None,
            reply: Some(reply_data.into()),
            tags: None,
            text: message,
        })
        .await?;
    Ok(result)
}

async fn get_post_thread(
    user_handle: Handle,
    record_key: String,
) -> Result<get_post_thread::OutputData, anyhow::Error> {
    let result = PublicAtAgent::default()
        .app_bsky_feed_getPostThread(get_post_thread::ParametersData {
            depth: None,
            parent_height: None,
            uri: format!(
                "at://{}/app.bsky.feed.post/{record_key}",
                user_handle.as_str()
            ),
        })
        .await?;
    Ok(result)
}
