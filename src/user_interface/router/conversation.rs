use crate::user_interface::router::AppRoute;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::user_interface::component::avatar::Avatar;

use crate::pds::agent::get_main_agent;
use crate::pds::agent::get_main_session;

use crate::pds::user_did::UserDid;
use crate::user_interface::component::loading::Loading;
use crate::user_interface::component::reload::Reload;
use anyhow::{anyhow, Error};
use atrium_api::chat::bsky::convo::defs::MessageInputData;
use atrium_api::chat::bsky::convo::get_messages::OutputMessagesItem;
use atrium_api::chat::bsky::convo::{get_messages, list_convos, send_message};
use atrium_api::types::string::Did;
use atrium_api::types::Union;
use bsky_sdk::BskyAgent;
use chrono::{DateTime, FixedOffset};

use crate::pds::store::SessionStoreOperator;
use crate::storage::session_store::get_session_store;
use crate::user_interface::component::select_account::{Direction, SelectAccount};
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

#[component]
pub fn ConversationPage(user_did: Did, conversation_id: String) -> Element {
    let mut messages_res = use_resource({
        to_owned![user_did, conversation_id];
        move || {
            to_owned![user_did, conversation_id];
            async move { get_messages(user_did, conversation_id).await }
        }
    });

    let mut busying = use_signal(|| false);

    let mut draft_signal = use_signal(String::new);

    let x = if let Some(messages) = messages_res.as_ref() {
        match messages.as_ref() {
            Ok(messages) => {
                let s = messages.messages.iter().rev().filter_map(|m| match m {
                    Union::Refs(m) => match m {
                        OutputMessagesItem::ChatBskyConvoDefsMessageView(mv) => {
                            let sender_did = mv.sender.did.to_owned();
                            if mv.text.trim().is_empty(){
                                return None;
                            }
                            if &sender_did == &user_did {
                                rsx! {
                                    div { class: "flex flex-row",
                                        span { class: "flex-1" }
                                        span { class: "max-w-3xl text-wrap p-1 bg-sky-500 rounded-lg text-slate-100",
                                            "{mv.text}"
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div {
                                        div {
                                            Avatar { user_did: UserDid(sender_did) }
                                        }
                                        div {
                                            span { class: "p-1 bg-gray-300 rounded-lg text-wrap",
                                                "{mv.text}"
                                            }
                                        }
                                    }
                                }
                            }.into()
                        }
                        OutputMessagesItem::ChatBskyConvoDefsDeletedMessageView(dmv) => {
                            None
                        }
                    },
                    Union::Unknown(_) => {
                        None
                    }
                });
                rsx! {
                    div { class: "size-full flex flex-col",
                        div { class: "flex-1 overflow-auto", {s} }
                        div { class: "flex flex-row",
                            Avatar {
                                user_did: UserDid(user_did.clone()),
                                with_display_name: false,
                                with_handle: false,
                            }
                            input {
                                class: "flex-1",
                                value: draft_signal,
                                onchange: move |evt| {
                                    draft_signal.set(evt.value());
                                },
                            }
                            span {
                                onclick: {
                                    to_owned![user_did, conversation_id];
                                    move |_| {
                                        debug!("send button onclick");
                                        to_owned![user_did, conversation_id];
                                        async move {
                                            let draft = { draft_signal.peek().to_string() };
                                            if false == { *busying.peek() } && false == draft.trim().is_empty() {
                                                busying.set(true);
                                                match send_message(user_did, conversation_id, draft).await {
                                                    Ok(()) => {
                                                        debug!("send message success");
                                                        *draft_signal.write() = String::new();
                                                        messages_res.restart();
                                                    }
                                                    Err(err) => {
                                                        debug!("send message error : {err}");
                                                    }
                                                }
                                                busying.set(false);
                                            }
                                        }
                                    }
                                },
                                "send"
                            }
                        }
                    }
                }
            }
            Err(err) => {
                rsx! { "loaded error : {err}" }
            }
        }
    } else {
        rsx! {
            Loading {}
        }
    };
    x
}

async fn send_message(
    did: Did,
    conversation_id: String,
    text: String,
) -> Result<(), anyhow::Error> {
    let session_store = get_session_store().await?;
    let agent = session_store.get_agent(did).await?;

    let messages = agent
        .chat_bsky_convo_sendMessageuse(send_message::InputData {
            convo_id: conversation_id,
            message: MessageInputData {
                embed: None,
                facets: None,
                text,
            }
            .into(),
        })
        .await?;

    Ok(())
}

async fn get_messages(
    did: Did,
    conversation_id: String,
) -> Result<get_messages::Output, anyhow::Error> {
    let session_store = get_session_store().await?;
    let agent = session_store.get_agent(did).await?;

    let messages = agent
        .chat_bsky_convo_getMessages(get_messages::ParametersData {
            convo_id: conversation_id,
            cursor: None,
            limit: None,
        })
        .await?;

    Ok(messages)
}
