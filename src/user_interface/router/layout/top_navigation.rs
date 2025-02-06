use std::sync::atomic::Ordering;

use dioxus::prelude::*;

use dioxus_router::prelude::{use_navigator, Outlet};

use crate::imgs::BACK_ICON_50_50;
use crate::user_interface::router::AppRoute;

pub const TOP_NAVIGATION_HEIGHT: usize = 48;
pub const TOP_NAVIGATION_ITEM_HEIGHT: usize = 24;

pub fn TopNavigation() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col",
            div { class: "bg-white h-12 w-full",
                GoBackButton {
                    img { class: "size-6 m-3", src: BACK_ICON_50_50 }
                }
            }
            div { class: "flex-1 overflow-y-scroll", Outlet::<AppRoute> {} }
        }
    }
}
