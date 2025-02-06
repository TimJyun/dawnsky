use crate::user_interface::client_util::get_user_language;
use atrium_api::types::string::Handle;

use crate::imgs::DELETE_TRASH_ICON_50_50;
use crate::imgs::PEN_ICON_50_50;

use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

use tracing::debug;

use crate::user_interface::router::AppRoute;
use chrono::{DateTime, FixedOffset};
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::pds::agent::get_main_agent;

use crate::i18n::Language;
use crate::pds::user_did::UserDid;
use crate::user_interface::component::post_view::PostView;
use atrium_api::app::bsky::feed::{get_actor_feeds, get_author_feed, post};
use atrium_api::types::TryFromUnknown;
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use crate::pds::noauth::PublicAtAgent;
use std::cell::Ref;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;

#[component]
pub fn UserPosts(user_did: UserDid) -> Element {
    let list_res = use_resource(move || {
        to_owned![user_did];

        async move {
            Ok::<_, anyhow::Error>(
                PublicAtAgent::default()
                    .app_bsky_feed_getAuthorFeed(get_author_feed::ParametersData {
                        actor: user_did.0.into(),
                        cursor: None,
                        filter: None,
                        include_pins: None,
                        limit: None,
                    })
                    .await?,
            )
        }
    });

    let x = match list_res.read().as_ref() {
        Some(list) => match list.as_ref() {
            Ok(l) => {
                let list_node = l.feed.iter().map(|p| {
                    rsx! {
                        PostView { post_view: p.post.data.to_owned() }
                    }
                });

                rsx! {
                    {list_node}
                }
            }
            Err(err) => {
                rsx! { "load LastUpdateList error : {err}" }
            }
        },
        None => {
            rsx! { "loading" }
        }
    };
    x
}
