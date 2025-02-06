use crate::pds::agent::get_main_agent;
use crate::pds::noauth::PublicAtAgent;
use crate::pds::user_did::UserDid;
use crate::user_interface::router::AppRoute;
use anyhow::anyhow;
use atrium_api::app::bsky::actor::get_profile;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use std::borrow::ToOwned;

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    user_did: UserDid,
    #[props(default = true)]
    with_display_name: bool,
    #[props(default = true)]
    with_handle: bool,
}

pub fn Avatar(props: AvatarProps) -> Element {
    let AvatarProps {
        user_did,
        with_display_name,
        with_handle,
    } = props;

    let profile = use_resource(move || {
        let actor = user_did.to_owned();
        async move {
            let parameters = get_profile::ParametersData {
                actor: actor.0.into(),
            };
            let output = if let Ok(agent) = get_main_agent().await {
                agent.app_bsky_actor_getProfile(parameters).await
            } else {
                PublicAtAgent::default()
                    .app_bsky_actor_getProfile(parameters)
                    .await
            };
            Ok::<_, anyhow::Error>(output?)
        }
    });

    let profile_read = profile.read();

    let avatar = profile_read
        .as_ref()
        .map(|p| p.as_ref().map(|p| p.avatar.clone()).ok())
        .flatten()
        .flatten()
        .unwrap_or_else(String::new);

    let display_name = if with_display_name {
        profile_read
            .as_ref()
            .map(|p| p.as_ref().map(|p| p.display_name.clone()).ok())
            .flatten()
            .flatten()
    } else {
        None
    };

    let handle = if with_handle {
        profile_read
            .as_ref()
            .map(|p| p.as_ref().map(|p| p.handle.clone()).ok())
            .flatten()
            .map(|h| {
                rsx! {
                    Link {
                        to: AppRoute::ProfilePage {
                            user_handle: h.to_owned(),
                        },
                        span { class: "text-sm text-gray-400", "@{h.as_str()}" }
                    }
                }
            })
    } else {
        None
    };

    rsx! {

        img { class: "size-6 inline-block rounded-full", src: "{avatar}" }
        {display_name}
        {handle}
    }
}
