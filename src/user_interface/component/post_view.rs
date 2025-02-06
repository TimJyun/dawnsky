use crate::user_interface::client_util::get_user_language;
use atrium_api::types::string::{Handle, Nsid};
use std::borrow::Borrow;

use crate::imgs::{
    DATA_TRANSFER_ICON_50_50, DELETE_TRASH_ICON_50_50, LIKE_ICON_50_50, TALK_ICON_50_50,
};
use crate::imgs::{PEN_ICON_50_50, TRASH_ICON_50_50};

use crate::pds::record::Record;
use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

use tracing::debug;

use crate::user_interface::router::AppRoute;
use chrono::{DateTime, FixedOffset};
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::i18n::Language;
use atrium_api::app::bsky::feed::defs::{FeedViewPost, PostViewData};
use atrium_api::app::bsky::feed::{get_actor_feeds, get_author_feed, post};
use atrium_api::types::TryFromUnknown;
use dioxus::prelude::*;
use dioxus::CapturedError;

use serde::{Deserialize, Serialize};

use crate::pds::agent::{get_main_agent, get_main_session_did};
use crate::pds::store::{SessionStore, SessionStoreOperator};
use crate::storage::session_store::get_session_store;
use crate::user_interface::app::refresh_app;
use crate::user_interface::component::confirm_box::confirm;
use crate::util::sleep::sleep;
use atrium_api::app::bsky::feed::post::RecordData;
use atrium_api::com::atproto::repo::delete_record;
use std::cell::Ref;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;
use std::str::FromStr;

#[component]
pub fn PostView(post_view: PostViewData) -> Element {
    let my_dids = use_resource(|| async move {
        if let Ok(session_store) = get_session_store().await {
            session_store.list_sessions().await.unwrap_or_default()
        } else {
            Vec::new()
        }
    });

    let author = post_view
        .author
        .display_name
        .as_ref()
        .cloned()
        .unwrap_or(String::from("unknown"));

    let avatar = if let Some(avatar) = post_view.author.avatar.as_ref() {
        rsx! {
            img { class: "size-6 inline-block", src: "{avatar}" }
        }
    } else {
        rsx! {}
    };
    let handle = post_view.author.handle.as_str();

    let record = post::Record::try_from_unknown(post_view.record.to_owned());

    let reply_count = post_view.reply_count.map(|reply_count| {
        rsx! {
            img { class: "size-6 inline-block", src: TALK_ICON_50_50 }
            "{reply_count}"
        }
    });
    let repost_count = post_view.repost_count.map(|repost_count| {
        rsx! {
            img { class: "size-6 inline-block", src: DATA_TRANSFER_ICON_50_50 }
            "{repost_count}"
        }
    });
    let like_count = post_view.like_count.map(|like_count| {
        rsx! {
            img { class: "size-6 inline-block", src: LIKE_ICON_50_50 }
            "{like_count}"
        }
    });

    let post_record = post::RecordData::try_from_unknown(post_view.record.clone()).unwrap();

    let delete = if my_dids
        .read()
        .as_ref()
        .map(|v| v.contains(&post_view.author.did))
        .unwrap_or(false)
    {
        rsx! {
            span {
                class: "size-full",
                onclick: {
                    let author = post_view.author.did.clone();
                    let rkey = aturi2rkey(post_view.uri.as_str()).map(str::to_string);
                    move |_| {
                        to_owned![author, rkey];
                        async move {
                            if confirm(["do you want to delete the post"]).await {
                                if let Ok(session_store) = get_session_store().await {
                                    if let Ok(agent) = session_store.get_agent(author).await {
                                        match agent
                                            .com_atproto_repo_deleteRecord(delete_record::InputData {
                                                collection: Nsid::from_str(RecordData::NSID).unwrap(),
                                                repo: agent.get_session().await.unwrap().did.clone().into(),
                                                rkey: rkey.unwrap(),
                                                swap_commit: None,
                                                swap_record: None,
                                            })
                                            .await
                                        {
                                            Ok(_) => {
                                                debug!("delete record success");
                                                sleep(500).await;
                                                refresh_app();
                                            }
                                            Err(err) => {
                                                debug!("delete record failed : {err}");
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                },
                img { class: "size-6 inline-block", src: TRASH_ICON_50_50 }
                "delete"
            }
        }
    } else {
        rsx! {}
    };

    let counts = rsx! {
        div { class: "flex flex-row",
            span { class: "flex-1", {reply_count} }
            span { class: "flex-1", {repost_count} }
            span { class: "flex-1", {like_count} }
            span { class: "flex-1", {delete} }
        }
    };

    let content = record
        .map(|r| {
            rsx! { "{r.text}" }
        })
        .unwrap_or_else(|_| {
            rsx! {
                span { class: "text-gray-400", "(post is empty)" }
            }
        });

    let post_json = serde_json::to_string(&post_view).unwrap();

    let (user_handle, record_key) = (
        post_view.author.handle.to_owned(),
        post_view
            .uri
            .split("/")
            .last()
            .unwrap_or("")
            .split("?")
            .next()
            .unwrap_or("")
            .to_string(),
    );

    rsx! {
        div { class: "border",
            div {
                Link {
                    to: AppRoute::ProfilePage {
                        user_handle: Handle::new(handle.to_string()).unwrap(),
                    },
                    span { {avatar} }
                    span { "{author}" }
                    " "
                    span { class: "text-sm text-gray-400", "@{handle}" }
                }
            }
            Link {
                to: AppRoute::PostPage {
                    user_handle,
                    record_key,
                },
                div { {content} }
            }
            div { {counts} }
        }
    }
}

pub fn aturi2rkey(aturi: &str) -> Option<&str> {
    aturi.split("/").last()?.split("#").next()
}
