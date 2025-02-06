use dioxus::prelude::*;
use std::cell::Ref;
use std::collections::HashMap;

use crate::user_interface::router::AppRoute;

use atrium_api::com::atproto::sync::list_repos;
use atrium_api::com::atproto::sync::list_repos::Output;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::storage::pds_store::get_pds_admin_agent;
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

pub fn PdsUsers() -> Element {
    let list = use_resource(get_list);

    let x = if let Some(list) = list.read().as_ref() {
        match list {
            Ok(list) => {
                let list = list.repos.iter().map(|repo| {
                    let did = repo.did.as_str();

                    let status = repo
                        .status
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| "unknown");
                    rsx! {
                        div { class: "border",
                            div { "did": {did} }
                            div { "status": {status} }
                        }
                    }
                });

                rsx! {
                    {list}
                }
            }
            Err(err) => {
                rsx! { "load error : {err}" }
            }
        }
    } else {
        rsx! { "loading" }
    };
    x
}

async fn get_list() -> Result<list_repos::Output, anyhow::Error> {
    let agent = get_pds_admin_agent().await?;

    let repos = agent
        .com_atproto_sync_listRepos(list_repos::ParametersData {
            cursor: None,
            limit: None,
        })
        .await?;

    Ok(repos)
}
