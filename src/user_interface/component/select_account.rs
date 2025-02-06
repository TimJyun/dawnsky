use crate::pds::agent::{get_main_agent, get_main_session, get_main_session_did};
use crate::pds::noauth::PublicAtAgent;
use crate::pds::store::{SessionStore, SessionStoreOperator};
use crate::pds::user::Session;
use crate::pds::user_did::UserDid;
use crate::storage::session_store::get_session_store;
use crate::user_interface::router::AppRoute;
use anyhow::anyhow;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::types::string::Did;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use gloo_storage::LocalStorage;
use std::borrow::ToOwned;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[component]
pub fn SelectAccount(
    selected: Signal<Option<Did>>,
    direction: Direction,
    mut options: Option<BTreeSet<UserDid>>,
) -> Element {
    let options = use_resource(move || {
        let options = options.clone();
        async move {
            let options = async move {
                let session_store = get_session_store().await?;
                let sessions = session_store
                    .list_sessions()
                    .await?
                    .into_iter()
                    .map(|d| UserDid(d));
                let sessions = BTreeSet::from_iter(sessions);

                let options = if let Some(mut options) = options {
                    options.retain(|o| sessions.contains(o));
                    options
                } else {
                    sessions
                };
                Ok::<BTreeSet<UserDid>, anyhow::Error>(options)
            }
            .await
            .unwrap_or_default();

            let selected_is_none = { selected.peek().is_none() };
            if selected_is_none {
                *selected.write() = options.first().map(|uid| uid.0.clone());
            }
            options
        }
    })
    .suspend()?;

    if selected.read().is_none() {
        return rsx! {};
    }

    let mut opened = use_signal(|| false);
    let main_avatar = use_resource(move || async move {
        async move {
            let session_store = get_session_store().await.ok()?;
            let agent = session_store
                .get_agent(selected.read().clone().unwrap())
                .await
                .ok()?;
            agent
                .app_bsky_actor_getProfile(get_profile::ParametersData {
                    actor: agent.did.clone().into(),
                })
                .await
                .map(|o| o.data.avatar.unwrap_or_default())
                .ok()
        }
        .await
        .unwrap_or_default()
    })
    .suspend()?;

    let onclick = move |_| {
        let new_value = !{ *opened.peek() };
        *opened.write() = new_value;
    };

    let main_avatar = rsx! {
        div { class: "inline-block", onclick,
            img { class: "size-6 inline-block rounded-full", src: main_avatar }
        }
    };

    if *opened.read() {
        let direction = if direction == Direction::Down {
            ""
        } else {
            "top-0 -translate-y-full"
        };
        let list = rsx! {
            SuspenseBoundary { fallback: |context: SuspenseContext| rsx! {},
                div { class: "absolute bg-white {direction}",
                    SelectAccountList {
                        selected,
                        opened,
                        options: options.read().clone(),
                    }
                }
            }
        };
        rsx! {
            div { class: "inline-block relative",
                {main_avatar}
                {list}
            }
        }
    } else {
        main_avatar
    }
}

#[component]
fn SelectAccountList(
    selected: Signal<Option<Did>>,
    opened: Signal<bool>,
    options: BTreeSet<UserDid>,
) -> Element {
    let options = Arc::new(options);
    let session_avatar_name = use_resource(move || {
        let options = options.clone();
        async move {
            let session_store = get_session_store().await?;
            let mut avatars = Vec::with_capacity(options.len());
            for UserDid(did) in options.iter() {
                if let Ok(agent) = session_store.get_agent(did.clone()).await {
                    let avatar_name = agent
                        .app_bsky_actor_getProfile(get_profile::ParametersData {
                            actor: agent.did.clone().into(),
                        })
                        .await
                        .map(|o| {
                            (
                                o.data.avatar.unwrap_or_default(),
                                o.data
                                    .display_name
                                    .unwrap_or_else(|| o.data.handle.to_string()),
                            )
                        })
                        .unwrap_or_default();

                    avatars.push((agent.session, avatar_name.0, avatar_name.1));
                };
            }
            Ok::<_, anyhow::Error>(avatars)
        }
    })
    .suspend()?;

    let x = match session_avatar_name.read().as_ref() {
        Ok(session_avatar_name) => {
            let accounts = session_avatar_name
                .iter()
                .map(|(ref session, ref avatar, ref name)| {
                    let did = session.did.clone();
                    rsx! {
                        div {
                            class: "flex flex-row bg-white",
                            onclick: move |_| {
                                *selected.write() = Some(did.clone());
                                opened.set(false);
                            },
                            img {
                                class: "size-6 inline-block rounded-full",
                                src: avatar.to_string(),
                            }
                            span { class: "bg-white", "{name}" }
                        }
                    }
                });

            rsx! {
                {accounts}
            }
        }
        Err(_) => {
            rsx! {}
        }
    };

    x
}
