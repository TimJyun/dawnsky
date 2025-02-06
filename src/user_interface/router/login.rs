use dioxus::prelude::*;
use std::cell::Ref;

use crate::user_interface::router::AppRoute;

use crate::pds::agent::{get_main_session, login};

use atrium_api::app::bsky::feed::get_timeline;
use atrium_api::types::string::Handle;
use bsky_sdk::BskyAgent;

use crate::pds::store::SessionStore;
use crate::pds::user_did::UserDid;
use crate::storage::session_store::get_session_store;
use crate::user_interface::component::avatar::Avatar;
use anyhow::Error;
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

pub fn LoginPage() -> Element {
    let nav = use_navigator();

    let mut handle = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut busying = use_signal(|| false);

    //设置id是为了提醒浏览器密码管理器
    rsx! {
        div {
            div {
                "handle:"
                input {
                    id: "login-handle",
                    name: "login-handle",
                    onchange: move |evt| {
                        handle.set(evt.value().to_string());
                    },
                }
            }
            div {
                "password:"
                input {
                    id: "login-password",
                    name: "login-password",
                    r#type: "password",
                    onchange: move |evt| {
                        password.set(evt.value().to_string());
                    },
                }
            }
            div {
                input {
                    r#type: "button",
                    disabled: *busying.read(),
                    value: "login",
                    onclick: move |evt| {
                        busying.set(true);
                        async move {
                            debug!("try login");
                            if let Ok(handle) = Handle::new(handle.peek().trim().to_string()) {
                                let password = password.peek().as_str().to_string();
                                match login(handle, password).await {
                                    Ok(()) => {
                                        nav.replace(AppRoute::SocialPage {});
                                    }
                                    Err(err) => debug!("login error:{err}"),
                                }
                            } else {
                                debug!("Invalid handle");
                            }
                            busying.set(false);
                        }
                    },
                }
            }
        }
    }
}
