use dioxus::prelude::*;

use crate::text::TEXT;
use crate::user_interface::router::AppRoute;
use dioxus_router::prelude::{Link, Outlet};
pub const BOTTOM_NAVIGATION_HEIGHT: usize = 64;
pub const BOTTOM_NAVIGATION_ITEM_HEIGHT: usize = 32;

pub fn Navigation() -> Element {
    rsx! {
        div { style: "width:100%;height: 100%;display:flex;flex-direction:column;",
            div { style: "flex:1;overflow: auto;", Outlet::<AppRoute> {} }
            div { style: "
                    background-color: #fff;
                    height:{BOTTOM_NAVIGATION_HEIGHT}px;
                    width:100%;
                    display:flex;
                    align-items: center;
                    ",
                Link {
                    to: AppRoute::SocialPage {},
                    style: "flex:1;display: inline-block;text-align: center;",
                    "social"
                }
                Link {
                    to: AppRoute::MessagePage {},
                    style: "flex:1;display: inline-block;text-align: center;",
                    {TEXT.read().bottom_to_message_page}
                }
                Link {
                    to: AppRoute::MyPage {},
                    style: "flex:1;display: inline-block;text-align: center;",
                    {TEXT.read().bottom_to_my_page}
                }
            }
        }
    }
}
