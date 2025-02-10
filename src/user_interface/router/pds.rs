use std::collections::{BTreeMap, HashMap};
use std::mem::swap;
use std::ops::Deref;

pub mod users_management;
use dioxus::prelude::*;

use crate::user_interface::router::AppRoute;

use crate::pds::agent::get_main_session;
use crate::pds::store::SessionStoreOperator;
use crate::pds::{PdsDomain, PdsPassword};
use crate::storage::session_store::get_session_store;
use atrium_api::com::atproto::server::{create_account, create_invite_code};
use atrium_api::types::string::{Did, Handle};
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub type PdsStore = BTreeMap<PdsDomain, PdsPassword>;

static PDS_STORE_KEY: &str = "pds_store_key";

pub fn use_pds_store() -> Signal<PdsStore> {
    use_synced_storage::<LocalStorage, _>(PDS_STORE_KEY.to_string(), PdsStore::new)
}

pub fn PdsPage() -> Element {
    let mut pds_store = use_pds_store();

    let mut pds_signal = use_signal(String::new);

    let pds_store_read = pds_store.read();

    let pds_list = pds_store_read.iter().map(|(domain, _)| {
        rsx! {
            div {
                Link {
                    to: AppRoute::PdsUsersManagementPage {
                        pds_domain: domain.to_string(),
                    },
                    "{domain}"
                }
                input {
                    class: "border",
                    r#type: "button",
                    value: "remove",
                    onclick: {
                        to_owned![domain];
                        move |_| {
                            pds_store.write().remove(&domain);
                        }
                    },
                }
            }
        }
    });

    rsx! {
        div {
            input {
                onchange: move |evt| {
                    *pds_signal.write() = evt.value();
                },
            }
            input {
                r#type: "button",
                value: "add",
                onclick: move |evt| {
                    let mut pds = String::new();
                    swap(&mut pds, &mut *pds_signal.write());
                    pds_store.write().entry(pds).or_insert(String::new());
                },
            }
        }
        div { {pds_list} }
    }
}
