use crate::pds::agent::{get_main_agent, get_main_session, MAIN_SESSION_DID};
use crate::user_interface::router::AppRoute;
use dioxus::prelude::*;

use crate::imgs::WRITE_ICON_50_50;
use crate::storage::session_store::get_session_store;
use crate::user_interface::app::refresh_app;
use std::cell::Ref;

use crate::imgs::{
    DOLLAR_ICON_50_50, GIFT_BOX_ICON_50_50, SETTING_ICON_50_50, SIGN_OUT_ICON_50_50,
};
use crate::util::sleep::sleep;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::types::string::Handle;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::pds::store::SessionStore;
use crate::user_interface::component::confirm_box::confirm;
use gloo_storage::{LocalStorage, Storage};
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

pub fn MyPage() -> Element {
    let nav = use_navigator();

    let mut user_profile_res = use_resource(move || async {
        (|| async {
            let agent = get_main_agent().await?;
            let user_did = get_main_session().await.map(|s| s.did.to_owned())?;
            let output = agent
                .app_bsky_actor_getProfile(get_profile::ParametersData {
                    actor: user_did.into(),
                })
                .await?;

            Ok::<_, anyhow::Error>(output)
        })()
        .await
    });

    let avatar = user_profile_res
        .as_ref()
        .map(|profile| {
            profile
                .as_ref()
                .ok()
                .map(|profile| profile.avatar.as_ref().map(|avatar| avatar.to_string()))
        })
        .flatten()
        .flatten()
        .unwrap_or_else(|| String::new());

    let display_name = user_profile_res
        .as_ref()
        .map(|profile| {
            profile.as_ref().ok().map(|profile| {
                profile
                    .display_name
                    .as_ref()
                    .map(|display_name| rsx! { "{display_name}" })
            })
        })
        .flatten()
        .flatten()
        .unwrap_or_else(|| {
            rsx! {
                span { class: "text-gray-400", "unknown" }
            }
        });

    let handle = user_profile_res
        .as_ref()
        .map(|profile| {
            profile
                .as_ref()
                .ok()
                .map(|profile| profile.handle.as_str().to_owned())
        })
        .flatten();

    let handle_node = handle
        .to_owned()
        .map(|handle| {
            rsx! { "{handle.as_str()}" }
        })
        .unwrap_or_else(|| {
            rsx! { "loading" }
        });

    let login_state_nav_node = match user_profile_res.as_ref() {
        Some(user_info) => {
            //loaded
            match user_info.as_ref() {
                Ok(user_info) => {
                    rsx! {
                        div {
                            class: "m-4 w-full text-center",
                            onclick: move |_| {
                                if let Some(handle) = handle.to_owned() {
                                    if let Ok(user_handle) = Handle::new(handle) {
                                        nav.push(AppRoute::ProfilePage {
                                            user_handle,
                                        });
                                    }
                                }
                            },
                            div { class: "inline-block my-auto h-16",
                                img {
                                    class: "rounded-full size-16 mx-1",
                                    src: avatar,
                                }
                            }
                            div { class: "inline-block my-auto h-16",
                                div { class: "mx-1 text-2xl", {display_name} }
                                div { class: "mx-1", {handle_node} }
                            }
                        }
                    }
                }
                Err(err) => {
                    rsx! {
                        div {
                            Link { to: AppRoute::LoginPage {}, "login" }
                        }
                        div {
                            Link { to: AppRoute::SignupPage {}, "signup" }
                        }
                    }
                }
            }
        }
        None => {
            rsx! { "loading" }
        }
    };

    rsx! {
        div { class: "border m-12 h-40", {login_state_nav_node} }
        div { class: "p-8",
            Link { to: AppRoute::SettingPage {},
                img { class: "size-6 inline-block", src: SETTING_ICON_50_50 }
                span { class: "text-xl", "Setting" }
            }
        }
        div { class: "p-8 text-red-600",
            span {
                onclick: move |_| {
                    async move {
                        if confirm(["you will be signed out of all your accounts"]).await {
                            if let Ok(session_store) = get_session_store().await {
                                if let Ok(dids) = session_store.list_sessions().await {
                                    for did in dids.into_iter() {
                                        let _ = session_store.remove_session(did).await;
                                    }
                                    let _ = LocalStorage::delete(MAIN_SESSION_DID);
                                    refresh_app();
                                }
                            }
                        }
                    }
                },
                img { class: "size-6 inline-block", src: SIGN_OUT_ICON_50_50 }
                span { class: "text-xl", "Sign Out" }
            }
        }
    }
}

#[component]
fn BalanceComponent(neutron: i64, proton: i64) -> Element {
    rsx! {
        div { class: "flex flex-row m-4",
            span { class: "flex-1 text-center",
                div { class: "h-4 w-full text-center", "{neutron}" }
                div { class: "h-4 w-full text-center", "neutron" }
            }
            span { class: "flex-1 text-center",
                div { class: "h-4 w-full text-center", "{proton}" }
                div { class: "h-4 w-full text-center", "proton" }
            }
        }
    }
}
