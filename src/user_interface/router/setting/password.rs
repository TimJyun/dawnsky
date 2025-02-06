use crate::util::sleep::sleep;
use dioxus::prelude::*;

use std::cell::Ref;

use crate::user_interface::client_util::go_back_or_replace_to_index;

use dioxus::html::completions::CompleteWithBraces::text;
use dioxus_html::FileEngine;
use opendal::Operator;

use std::ops::Deref;
use std::sync::Arc;

use tracing::debug;

use crate::i18n::Language;
use crate::user_interface::router::AppRoute;
use dioxus::prelude::*;

pub fn PasswordPage() -> Element {
    rsx! {}
}

// pub fn PasswordPage() -> Element {
//     let nav = use_navigator();
//     let mut busying = use_signal(|| false);
//     let mut user_info_res = use_resource(get_user_did);
//
//     let mut current_password = use_signal(String::new);
//     let mut new_password = use_signal(String::new);
//     let mut confirm_password = use_signal(String::new);
//
//     rsx! {
//         div { class: "m-1 border",
//             label {
//                 div { class: "text-stone-400", "Current password" }
//                 div {
//                     input {
//                         r#type: "password",
//                         onchange: move |evt| {
//                             current_password.set(evt.value());
//                         },
//                     }
//                 }
//             }
//         }
//         div { class: "m-1 border",
//             label {
//                 div { class: "text-stone-400", "New password" }
//                 div {
//                     input {
//                         r#type: "password",
//                         onchange: move |evt| {
//                             new_password.set(evt.value());
//                         },
//                     }
//                 }
//             }
//         }
//         div { class: "m-1 border",
//             label {
//                 div { class: "text-stone-400", "Confirm password" }
//                 div {
//                     input {
//                         r#type: "password",
//                         onchange: move |evt| {
//                             confirm_password.set(evt.value());
//                         },
//                     }
//                 }
//             }
//         }
//         div {
//             input {
//                 class: "btn-primary",
//                 r#type: "button",
//                 value: "Save",
//                 disabled: *busying.read() || current_password.read().is_empty()
//                     || new_password.read().is_empty() || confirm_password.read().is_empty()
//                     || (new_password.read().as_str() != confirm_password.read().as_str()),
//                 onclick: move |evt| {
//                     debug!("button save onclick");
//                     busying.set(true);
//                     spawn(async move {
//                         let result = set_user_password(
//                                 get_passwd_hash(current_password.peek().as_str()),
//                                 get_passwd_hash(new_password.peek().as_str()),
//                             )
//                             .await;
//                         if let Err(err) = result {
//                             debug!("change password error : {err}");
//                         } else {
//                             debug!("change password success");
//                         }
//                         busying.set(false);
//                     });
//                 },
//             }
//         }
//     }
// }
//
// #[server(input = Cbor)]
// async fn set_user_password(
//     current_password_hash: String,
//     new_password_hash: String,
// ) -> Result<(), ServerFnError> {
//     use crate::database::init::get_global_database;
//     let db = get_global_database()?;
//     let user_did = get_user_did().await?;
//     let current_password_hash2 = get_passwd_hash(current_password_hash);
//     let new_password_hash2 = get_passwd_hash(new_password_hash);
//     let result = user::Entity::update_many()
//         .filter(user::Column::UserDid.eq(user_did))
//         .filter(user::Column::PasswdHash2.eq(current_password_hash2.to_string()))
//         .col_expr(user::Column::PasswdHash2, Expr::value(new_password_hash2))
//         .exec(db)
//         .await?;
//
//     if result.rows_affected > 0 {
//         let _r = session::Entity::update_many()
//             .filter(session::Column::UserDid.eq(user_did))
//             .col_expr(session::Column::Active, Expr::value(false))
//             .exec(db)
//             .await;
//         Ok(())
//     } else {
//         Err(ServerFnError::new(
//             "current password error : rows_affected = 0",
//         ))
//     }
// }
