use crate::pds::PUBLIC_AGENT;
use crate::user_interface::component::user_info::UserInfo;

use crate::pds::agent::get_main_agent;
use crate::storage::pds_store::get_pds_admin_agent;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::com::atproto::admin::{disable_invite_codes, get_invite_codes};
use atrium_api::com::atproto::server::create_invite_code;
use atrium_api::com::atproto::sync::list_repos;
use atrium_api::com::atproto::sync::list_repos::Output;
use atrium_api::types::string::AtIdentifier;
use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::str::FromStr;

pub fn InviteCodesPage() -> Element {
    let mut invite_codes_res = use_resource(|| async {
        //todo cursor
        get_pds_admin_agent()
            .await?
            .com_atproto_admin_getInviteCodes(get_invite_codes::ParametersData {
                cursor: None,
                limit: None,
                sort: None,
            })
            .await
    });

    let mut use_count = use_signal(|| 1usize);

    let x = if let Some(a) = invite_codes_res.read().as_ref() {
        match a.as_ref() {
            Ok(a) => {
                let codes = a.codes.iter().map(|c| {
                    let disabled = if c.disabled {
                        rsx! { "disabled : true" }
                    } else {
                        rsx! {
                            span {
                                class: "border",
                                onclick: {
                                    let code = c.code.to_string();
                                    move |_| {
                                        let code = code.to_string();
                                        async move {
                                            if let Ok(agent) = get_pds_admin_agent().await {
                                                let _ = agent
                                                    .com_atproto_admin_disableInviteCodes(disable_invite_codes::InputData {
                                                        accounts: None,
                                                        codes: Some(Vec::from([code.to_string()])),
                                                    })
                                                    .await;
                                                invite_codes_res.restart();
                                            }
                                        }
                                    }
                                },
                                "disable it"
                            }
                        }
                    };

                    rsx! {
                        div { class: "border",
                            div { class: "flex flex-row",
                                span { class: "flex-1", "code : {c.code}" }
                                span { class: "flex-1", "usage/available : {c.uses.len()}/{c.available}" }
                            }
                            div { class: "flex flex-row",
                                span { class: "flex-1", "created_at : {c.created_at.as_str()}" }

                                span { class: "flex-1", {disabled} }
                            }
                        }
                    }
                });
                rsx! {
                    {codes}
                }
            }
            Err(err) => {
                rsx! { "load error : {err}" }
            }
        }
    } else {
        rsx! { "loading" }
    };

    rsx! {
        div {

            span {
                input {
                    value: use_count.read().to_string(),
                    onchange: move |evt| {
                        let mut use_count_write = use_count.write();
                        if let Ok(count) = usize::from_str(&evt.value()) {
                            *use_count_write = count;
                        }
                    },
                }
            }
            span {
                class: "border",
                onclick: move |_| {
                    async move {
                        if let Ok(agent) = get_pds_admin_agent().await {
                            let _ = agent
                                .com_atproto_createInviteCode(create_invite_code::InputData {
                                    for_account: None,
                                    use_count: *use_count.read() as i64,
                                })
                                .await;
                            invite_codes_res.restart();
                        }
                    }
                },
                "create invite code"
            }
        }
        div { {x} }
    }
}
