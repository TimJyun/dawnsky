use crate::user_interface::component::confirm_box::ConfirmBox;
use crate::user_interface::component::loading::Loading;
use crate::user_interface::router::AppRoute;
use atrium_api::agent::Session;
use dioxus::core_macro::rsx;
use dioxus::dioxus_core::Element;
use dioxus::document::Link;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use dioxus_html::head;
use jwt_compact::UntrustedToken;
use opendal::{services, Operator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io;
use std::io::Error;
use tracing::debug;

pub(crate) static NEED_UPDATE: GlobalSignal<bool> = Signal::global(|| false);

pub(crate) fn app() -> Element {
    let css = rsx! {
        Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Link { rel: "stylesheet", href: asset!("/assets/custom.css") }
    };
    if { *NEED_UPDATE.read() == true } {
        spawn(async { *NEED_UPDATE.write() = false });
        return rsx! {
            {css}
        };
    }
    rsx! {
        {css}
        ConfirmBox {}
        SuspenseBoundary {
            fallback: |context: SuspenseContext| rsx! {
                // Render a loading placeholder if
                // any child component is "suspended"
                Loading {}
            },
            Router::<AppRoute> {}
        }
    }
}

pub(crate) fn reload_page() {
    if let Some(window) = web_sys::window() {
        let _ = window.location().reload();
    }
}

pub(crate) fn refresh_app() {
    *NEED_UPDATE.write() = true;
}
