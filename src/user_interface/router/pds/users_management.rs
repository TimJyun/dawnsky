use dioxus::prelude::*;
use std::cell::Ref;
use std::collections::HashMap;

use crate::user_interface::router::AppRoute;

use atrium_api::com::atproto::sync::list_repos;
use atrium_api::com::atproto::sync::list_repos::Output;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::pds::admin::PdsAdminAgent;
use crate::pds::noauth::PublicAtAgent;
use crate::pds::user_did::UserDid;
use crate::pds::PdsDomain;
use crate::storage::pds_store::get_pds_admin_agent;
use crate::user_interface::app::refresh_app;
use crate::user_interface::component::avatar::Avatar;
use crate::user_interface::router::pds::{use_pds_store, PdsStore};
use atrium_api::com::atproto::admin::defs::{RepoRefData, StatusAttrData};
use atrium_api::com::atproto::admin::update_subject_status;
use atrium_api::com::atproto::admin::update_subject_status::InputSubjectRefs;
use atrium_api::types::string::Did;
use atrium_api::types::Union;
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

#[component]
pub fn PdsUsersManagementPage(pds_domain: PdsDomain) -> Element {
    let pds_domain = use_memo(use_reactive((&pds_domain,), |(pds_domain,)| pds_domain));

    let list = use_resource(move || get_list(pds_domain.read().clone())).suspend()?;

    let mut pds_store = use_pds_store();

    let mut passwd = use_signal(move || {
        pds_store
            .peek()
            .get(pds_domain.peek().deref())
            .cloned()
            .unwrap_or_default()
    });

    let x = match list.read().as_ref() {
        Ok(list) => {
            let list = list.repos.iter().map(|repo| {
                let did = repo.did.as_str();
                let status = repo
                    .status
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or_else(|| "unknown");

                let active = repo
                    .active
                    .map(|active| active.to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let takedown_or_not = if repo.active.unwrap_or(true) {
                    rsx! {
                        span {
                            class: "border",
                            onclick: {
                                to_owned![did];
                                move |_| {
                                    to_owned![did];
                                    async move {
                                        if let Err(err) = takedown(pds_store, pds_domain, did.clone(), true)
                                            .await
                                        {
                                            debug!("takedown error : {err}");
                                        }
                                        refresh_app();
                                    }
                                }
                            },
                            "takedown"
                        }
                    }
                } else {
                    rsx! {
                        span {
                            onclick: {
                                to_owned![did];
                                move |_| {
                                    to_owned![did];
                                    async move {
                                        if let Err(err) = takedown(pds_store, pds_domain, did.clone(), false)
                                            .await
                                        {
                                            debug!("untakedown error : {err}");
                                        }
                                        refresh_app();
                                    }
                                }
                            },
                            "untakedown"
                        }
                    }
                };


               let status =   repo.status.clone().map(|status|{
                   rsx!{ "status : {status}" }
               });

                rsx! {
                    div { class: "border flex flex-row",
                        div { class: "flex-1",
                            div {
                                Avatar { user_did: UserDid(Did::new(did.to_string()).unwrap()) }
                            }
                            div { class: "flex flex-row",
                                span { class: "flex-1", "active : {active}" }
                                span { class: "flex-1", {status} }
                            }
                        }
                        div { {takedown_or_not} }
                    }
                }
            });

            rsx! {
                div {
                    div { "pds_domain : {pds_domain}" }
                    div {
                        "password : "
                        input {
                            value: passwd,
                            r#type: "password",
                            onchange: move |evt| {
                                *passwd.write() = evt.value();
                            },
                        }
                        input {
                            r#type: "button",
                            value: "save",
                            onclick: move |_| {
                                pds_store
                                    .write()
                                    .insert(pds_domain.peek().to_string(), passwd.peek().to_string());
                            },
                        }
                    }
                    {list}
                }
            }
        }
        Err(err) => {
            rsx! { "load error : {err}" }
        }
    };
    x
}

async fn get_list(pds_domain: PdsDomain) -> Result<list_repos::OutputData, anyhow::Error> {
    let repos = PublicAtAgent::default()
        .com_atproto_sync_listRepos(
            pds_domain,
            list_repos::ParametersData {
                cursor: None,
                limit: None,
            },
        )
        .await?;
    debug!("{}", serde_json::to_string(&repos).unwrap());

    Ok(repos)
}

async fn takedown(
    pds_store: Signal<PdsStore>,
    pds_domain: Memo<PdsDomain>,
    did: String,
    new_state: bool,
) -> Result<(), anyhow::Error> {
    let passwd = pds_store
        .peek()
        .get(&*pds_domain.peek())
        .cloned()
        .unwrap_or_default();

    PdsAdminAgent {
        pds_site: pds_domain.to_string(),
        token: passwd,
    }
    .com_atproto_admin_updateSubjectStatus(update_subject_status::InputData {
        subject: Union::Refs(InputSubjectRefs::ComAtprotoAdminDefsRepoRef(Box::new(
            RepoRefData {
                did: Did::new(did).unwrap(),
            }
            .into(),
        ))),
        deactivated: None,
        takedown: Some(
            StatusAttrData {
                applied: new_state,
                r#ref: None,
            }
            .into(),
        ),
    })
    .await?;

    Ok(())
}
