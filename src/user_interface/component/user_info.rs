use crate::imgs::DELETE_TRASH_ICON_50_50;
use crate::imgs::PEN_ICON_50_50;

use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

use tracing::debug;

use crate::user_interface::router::AppRoute;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::pds::PUBLIC_AGENT;

use crate::storage::pds_store::get_pds_admin_agent;
use crate::util::sleep::sleep;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::com::atproto::admin::defs::{RepoRefData, StatusAttr, StatusAttrData};
use atrium_api::com::atproto::admin::update_subject_status;
use atrium_api::com::atproto::admin::update_subject_status::InputSubjectRefs;
use atrium_api::types::string::{AtIdentifier, Did};
use atrium_api::types::Union;
use chrono::{Local, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::Ref;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;
use std::str::FromStr;

#[component]
pub fn UserInfo(at_id: AtIdentifier) -> Element {
    let mut profile_res = use_resource({
        to_owned![at_id];
        move || {
            to_owned![at_id];
            async move {
                PUBLIC_AGENT
                    .api
                    .app
                    .bsky
                    .actor
                    .get_profile(
                        get_profile::ParametersData {
                            actor: at_id.to_owned(),
                        }
                        .into(),
                    )
                    .await
            }
        }
    });

    let profile = profile_res.suspend()?;
    let x = if let Ok(profile) = profile.read().as_ref().cloned() {
        let mut avatar = Signal::new(profile.avatar.clone().unwrap_or_default());
        let mut display_name = Signal::new(profile.display_name.clone().unwrap_or_default());
        let mut handle = Signal::new(profile.handle.clone());
        let mut did = Signal::new(profile.did.clone());

        rsx! {
            div { class: "flex flex-row",
                img { class: "inline-block size-6", src: avatar }
                span { class: "flex-1",
                    div {
                        "{display_name} "
                        span { class: "text-sm text-gray-400", "@{handle.read().as_str()}" }
                    }
                    div { "{did.read().as_str()}" }
                }
                span {

                    div {
                        class: "border",
                        onclick: move |_| {
                            async move {
                                takedown(did.peek().clone()).await;
                                profile_res.restart();
                            }
                        },
                        "takedown"
                    }
                    div {
                        class: "border",
                        onclick: move |_| {
                            async move {
                                untakedown(did.peek().clone()).await;
                                profile_res.restart();
                            }
                        },
                        "untakedown"
                    }
                }
            }
        }
    } else {
        spawn(async move {
            sleep(2000).await;
            profile_res.restart();
        });
        rsx! {}
    };
    x
}

async fn takedown(user_did: Did) -> Result<(), anyhow::Error> {
    let _ = get_pds_admin_agent()
        .await?
        .com_atproto_admin_updateSubjectStatus(update_subject_status::InputData {
            deactivated: None,
            subject: Union::Refs(InputSubjectRefs::ComAtprotoAdminDefsRepoRef(Box::new(
                RepoRefData { did: user_did }.into(),
            ))),
            takedown: Some(
                StatusAttrData {
                    applied: true,
                    r#ref: Some(Local::now().timestamp().to_string()),
                }
                .into(),
            ),
        })
        .await;
    Ok(())
}

async fn untakedown(user_did: Did) -> Result<(), anyhow::Error> {
    let _ = get_pds_admin_agent()
        .await?
        .com_atproto_admin_updateSubjectStatus(update_subject_status::InputData {
            deactivated: None,
            subject: Union::Refs(InputSubjectRefs::ComAtprotoAdminDefsRepoRef(Box::new(
                RepoRefData { did: user_did }.into(),
            ))),
            takedown: Some(
                StatusAttrData {
                    applied: false,
                    r#ref: None,
                }
                .into(),
            ),
        })
        .await;
    Ok(())
}
