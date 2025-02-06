use atrium_api::app::bsky::feed::get_timeline::Output;
use atrium_api::app::bsky::feed::{get_author_feed, get_timeline};
use atrium_api::app::bsky::feed::{get_posts, post};
use atrium_api::com::atproto::identity::resolve_handle;
use atrium_api::com::atproto::server::reserve_signing_key;
use atrium_api::types::{TryFromUnknown, Unknown};
use std::borrow::Borrow;

use crate::user_interface::router::AppRoute;

use atrium_api::agent::Session;
use atrium_api::types::string::{Datetime, Did, Handle};
use bsky_sdk::agent::config::Config;
use dioxus::prelude::*;
use ipld_core::ipld::Ipld;

use serde::{Deserialize, Serialize};

use crate::pds::agent::get_main_agent;
use crate::pds::noauth::PublicAtAgent;
use crate::pds::store::SessionStore;
use crate::storage::session_store::get_session_store;
use crate::user_interface::app::refresh_app;
use crate::user_interface::component::loading::Loading;
use crate::user_interface::component::post_view::PostView;
use crate::user_interface::component::select_account::{Direction, SelectAccount};
use anyhow::anyhow;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::app::bsky::feed::defs::FeedViewPost;
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, error};

pub fn SocialPage() -> Element {
    let output = use_resource(|| async { get_my_timeline().await });

    let mut new_post = use_signal(String::new);

    let selected = use_signal(|| None);

    let is_login_res = use_resource(|| async {
        let session_store = get_session_store().await?;
        let session_count = session_store.list_sessions().await?.len();
        Ok::<_, anyhow::Error>(session_count > 0)
    });

    let is_login = is_login_res
        .read()
        .as_ref()
        .map(|is_login| *is_login.as_ref().unwrap_or(&false))
        .unwrap_or(false);

    let new_post_node = if is_login {
        rsx! {

            div { class: "flex flex-row",
                SelectAccount { selected, direction: Direction::Down }
                textarea {
                    class: "flex-1 border min-h-[4.5rem] resize-none",
                    value: new_post,
                    onchange: move |evt| {
                        *new_post.write() = evt.value();
                    },
                }
                input {
                    r#type: "button",
                    value: "send",
                    onclick: move |etc| {
                        async move {
                            if let Ok(agent) = get_main_agent().await {
                                if let Ok(_) = agent
                                    .com_atproto_repo_createRecord(post::RecordData {
                                        created_at: Datetime::now(),
                                        embed: None,
                                        entities: None,
                                        facets: None,
                                        labels: None,
                                        langs: None,
                                        reply: None,
                                        tags: None,
                                        text: new_post.to_string(),
                                    })
                                    .await
                                {
                                    refresh_app();
                                }
                            }
                        }
                    },
                }
            }
        }
        .into()
    } else {
        None
    };

    let x = if let Some(output) = output.read().as_ref() {
        match output.as_ref() {
            Ok(feeds) => {
                let r = feeds.iter().map(|p| {
                    rsx! {
                        PostView { post_view: p.post.data.to_owned() }
                    }
                });
                rsx! {
                    {new_post_node}
                    {r}
                }
            }
            Err(err) => {
                rsx! { "error:{err}" }
            }
        }
    } else {
        rsx! {
            Loading {}
        }
    };
    x
}
async fn get_my_timeline() -> Result<Vec<FeedViewPost>, anyhow::Error> {
    let timeline = if let Ok(agent) = get_main_agent().await {
        if let Ok(timeline) = agent
            .app_bsky_feed_getTimeline(atrium_api::app::bsky::feed::get_timeline::ParametersData {
                algorithm: None,
                cursor: None,
                limit: None,
            })
            .await
        {
            timeline.data.feed
        } else {
            get_default_timeline().await?
        }
    } else {
        get_default_timeline().await?
    };

    Ok(timeline)
}

async fn get_default_timeline() -> Result<Vec<FeedViewPost>, anyhow::Error> {
    Ok(PublicAtAgent::default()
        .app_bsky_feed_getAuthorFeed(get_author_feed::ParametersData {
            actor: Handle::from_str("bsky.app").unwrap().into(),
            cursor: None,
            filter: None,
            include_pins: None,
            limit: None,
        })
        .await?
        .data
        .feed)
}
