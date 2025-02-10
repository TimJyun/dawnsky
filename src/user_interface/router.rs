use crate::user_interface::router::pds::users_management::PdsUsersManagementPage;
use crate::user_interface::router::post::PostPage;

use crate::user_interface::router::conversation::ConversationPage;
use crate::user_interface::router::layout::navigation::Navigation;
use crate::user_interface::router::layout::top_navigation::TopNavigation;
use crate::user_interface::router::login::LoginPage;
use crate::user_interface::router::message::MessagePage;
use crate::user_interface::router::my::MyPage;
use crate::user_interface::router::profile::ProfilePage;
use crate::user_interface::router::setting::password::PasswordPage;
use crate::user_interface::router::setting::SettingPage;
use crate::user_interface::router::signup::SignupPage;
use crate::user_interface::router::social::SocialPage;

use crate::user_interface::router::pds::PdsPage;
use crate::util::sleep::sleep;
use atrium_api::types::string::{Did, Handle};
use derive_more::{Display, FromStr};
use dioxus::prelude::*;

use crate::pds::PdsDomain;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod conversation;
mod invite_code;
mod layout;
mod login;
mod message;
mod my;
mod pds;
mod post;
mod profile;
mod setting;
mod signup;
mod social;

#[derive(Routable, PartialEq, Clone, Debug)]
#[rustfmt::skip]
pub enum AppRoute {
    #[layout(Navigation)]
    #[route("/")]
    SocialPage{},
    #[route("/messages")]
    MessagePage{},
    #[route("/my")]
    MyPage{},
    #[end_layout]
    //
    #[layout(TopNavigation)]
    #[route("/setting")]
    SettingPage{},
    #[route("/setting/password")]
    PasswordPage{},
    #[route("/conversation/:user_did/:conversation_id")]
    ConversationPage{user_did:Did,conversation_id:String },


    #[route("/user/:user_handle/bsky.app/post/:record_key")]
    PostPage{
        user_handle: Handle,record_key:String
    },


    #[end_layout]
    //
    #[route("/login")]
    LoginPage {},
    #[route("/signup")]
    SignupPage {},
    #[route("/profile/:user_handle")]
    ProfilePage{ user_handle: Handle},
    //
    //
    //pds admin
    #[route("/pds/:pds_domain/users")]
    PdsUsersManagementPage{pds_domain: PdsDomain},
    #[route("/pds")]
    PdsPage{},
    //
    //
    //
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let nav = use_navigator();
    let _ = use_coroutine(move |_: UnboundedReceiver<()>| async move {
        sleep(2_000).await;
        nav.replace(AppRoute::SocialPage {});
    });

    rsx! {
        h1 { "Page Not Found" }
        div { "forward to index page in 2 second" }
        div {
            Link { to: AppRoute::SocialPage {}, "click to index" }
        }
    }
}
