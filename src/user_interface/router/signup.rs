use dioxus::prelude::*;

use crate::user_interface::router::AppRoute;

use crate::pds::agent::get_main_session;
use atrium_api::com::atproto::server::{create_account, create_invite_code};
use atrium_api::types::string::{Did, Handle};

use crate::pds::store::SessionStoreOperator;
use crate::storage::session_store::get_session_store;
use gloo_storage::{LocalStorage, Storage};
use std::ops::Deref;
use tracing::debug;

pub fn SignupPage() -> Element {
    let nav = use_navigator();
    let _ = use_resource(move || async move {
        if get_main_session().await.is_ok() {
            nav.replace(AppRoute::MyPage {});
        }
    });

    let mut handle = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut password2 = use_signal(String::new);
    let mut invite_code = use_signal(String::new);
    let mut busying = use_signal(|| false);

    rsx! {
        div {
            div {
                "handle:"
                input {
                    onchange: move |evt| {
                        handle.set(evt.value().to_string());
                    },
                }
            }
            div {
                "email:"
                input {
                    onchange: move |evt| {
                        email.set(evt.value().to_string());
                    },
                }
            }
            div {
                "password:"
                input {
                    r#type: "password",
                    onchange: move |evt| {
                        password.set(evt.value().to_string());
                    },
                }
            }
            div {
                "repeat-password:"
                input {
                    r#type: "password",
                    onchange: move |evt| {
                        password2.set(evt.value().to_string());
                    },
                }
            }
            div {
                "invite-code:"
                input {
                    r#id: "invite-code",
                    onchange: move |evt| {
                        invite_code.set(evt.value().to_string());
                    },
                }
            }
            div {
                input {
                    r#type: "button",
                    disabled: *busying.read() || password.read().as_str() != password2.read().as_str(),
                    value: "submit",
                    onclick: move |evt| {
                        busying.set(true);
                        async move {
                            if let Ok(session_store) = get_session_store().await {
                                let handle = handle.peek().trim().to_string();
                                let passwd = password.peek().to_string();
                                let email = email.peek().trim().to_string();
                                let invite_code = invite_code.peek().to_string();
                                let invite_code = if invite_code.is_empty() {
                                    None
                                } else {
                                    Some(invite_code)
                                };
                                match session_store
                                    .create_account(handle, passwd, email, invite_code)
                                    .await
                                {
                                    Ok(account_info) => {
                                        debug!(
                                            "success to creat account , handle : {} ", account_info
                                            .handle.as_str()
                                        );
                                        debug!("{}", serde_json::to_string(& account_info).unwrap());
                                        let _ = LocalStorage::set(
                                            crate::pds::agent::MAIN_SESSION_DID,
                                            &account_info.did,
                                        );
                                        nav.push(AppRoute::MyPage {});
                                    }
                                    Err(error) => {
                                        debug!("signup failed:{error}");
                                    }
                                }
                            }
                            busying.set(false);
                        }
                    },
                }
            }
        }
    }
}
