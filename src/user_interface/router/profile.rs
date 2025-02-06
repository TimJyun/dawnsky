use dioxus::prelude::*;

use crate::pds::agent::get_main_agent;

use crate::pds::user_did::UserDid;
use crate::user_interface::component::loading::Loading;
use crate::user_interface::component::user_posts::UserPosts;
use atrium_api::app::bsky::actor::defs::{ProfileViewDetailed, ProfileViewDetailedData};
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::app::bsky::actor::get_profile::ParametersData;
use atrium_api::com::atproto::identity::resolve_handle;
use atrium_api::types::string::{AtIdentifier, Handle};
use bsky_sdk::BskyAgent;
use chrono::{DateTime, FixedOffset};
use ipld_core::ipld::Ipld;

use crate::pds::noauth::PublicAtAgent;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::thread::Scope;
use tracing::debug;

pub enum ListType {
    Collection,
    Posts,
}

#[component]
pub fn ProfilePage(user_handle: Handle) -> Element {
    let user_info_res = use_resource(move || get_user_profile(user_handle.to_owned()));

    let user_info_read = user_info_res.read();

    if let Some(user_info) = user_info_read.as_ref() {
        match user_info {
            Ok(user_info) => {
                let handle = &user_info.handle;
                let display_name = user_info
                    .display_name
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| user_info.handle.to_string());

                let avatar = if let Some(avatar) = user_info.avatar.as_ref() {
                    rsx! {
                        img { class: "size-6 inline-block", src: "{avatar}" }
                    }
                } else {
                    rsx! {
                        span { "blank" }
                    }
                };

                let counts = {
                    let followers_count =
                        user_info.followers_count.as_ref().map(|followers_count| {
                            rsx! { "{followers_count}followers" }
                        });

                    let following_count = user_info.follows_count.as_ref().map(|following| {
                        rsx! { "{following}following" }
                    });
                    let posts_count = user_info.posts_count.as_ref().map(|posts_count| {
                        rsx! { "posts" }
                    });

                    rsx! {
                        div {
                            {followers_count}
                            " "
                            {following_count}
                            " "
                            {posts_count}
                        }
                    }
                };

                let bio = user_info.description.as_ref().map(|bio| {
                    rsx! {
                        div { "{bio}" }
                    }
                });
                let user_did = UserDid(user_info.did.to_owned());

                rsx! {
                    div { {avatar} }
                    div { {display_name} }
                    div {
                        "@"
                        {handle}
                    }
                    {counts}
                    {bio}
                    div {
                        UserPosts { user_did }
                    }
                }
            }
            Err(err) => {
                rsx! {
                    div { "load error : {err}" }
                }
            }
        }
    } else {
        rsx! {
            Loading {}
        }
    }
}

async fn get_user_profile(user_handle: Handle) -> Result<ProfileViewDetailed, anyhow::Error> {
    let params = get_profile::ParametersData {
        actor: AtIdentifier::Handle(user_handle),
    };
    let result = if let Ok(agent) = get_main_agent().await {
        agent.app_bsky_actor_getProfile(params).await?
    } else {
        PublicAtAgent::default()
            .app_bsky_actor_getProfile(params)
            .await?
    };

    let n = result.display_name.as_ref().cloned().unwrap_or_default();
    debug!("name:{n}");

    debug!("{}", result.did.to_string());
    Ok(result)
}
