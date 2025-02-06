use dioxus::prelude::*;
use std::collections::HashMap;

use crate::user_interface::router::AppRoute;

use crate::pds::agent::{get_main_agent, get_main_session, get_main_session_did};
use crate::user_interface::component::loading::Loading;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::chat::bsky::convo::defs::ConvoViewLastMessageRefs;
use atrium_api::chat::bsky::convo::{get_convo_for_members, list_convos};
use atrium_api::types::string::Did;
use atrium_api::types::Union;
use bsky_sdk::BskyAgent;
use chrono::{DateTime, FixedOffset};

use crate::pds::store::{SessionStore, SessionStoreOperator};
use crate::pds::user_did::UserDid;
use crate::storage::session_store::get_session_store;
use crate::user_interface::component::avatar::Avatar;
use futures::{AsyncReadExt, FutureExt};
use std::ops::Deref;
use std::str::FromStr;
use tracing::{debug, info};

pub fn MessagePage() -> Element {
    let mut uid_with_sessions = use_signal(Vec::new);
    let uid_with_sessions_future = use_future(move || async move {
        let dids = get_session_store().await?.list_sessions().await?;
        for did in dids.into_iter() {
            let convs = get_conversations(did.clone()).await;
            //这里注意不要让guard跨越await边界
            let mut uid_with_sessions_write = uid_with_sessions.write();
            uid_with_sessions_write.push((did.clone(), convs));
            drop(uid_with_sessions_write);
        }

        Ok::<_, anyhow::Error>(())
    });

    if uid_with_sessions.read().is_empty() && get_main_session_did().is_err() {
        return rsx! {
            div { class: "w-full h-full",
                Link { to: AppRoute::LoginPage {}, "got to login" }
            }
        };
    }
    let uid_with_sessions_binding = uid_with_sessions.read();
    let x = uid_with_sessions_binding.iter().map(|(user_did, o)| {
        let s = if let Ok(o) = o.as_ref() {
            let convos = o.convos.iter().map(|cv| {
                let avatar = cv
                    .members
                    .iter()
                    .filter(|m| &m.did != user_did)
                    .map(|member| member.avatar.as_ref().map(|a| a.to_string()))
                    .next()
                    .flatten()
                    .unwrap_or_default();

                let lm = cv
                    .last_message
                    .as_ref()
                    .map(|lm| match lm {
                        Union::Refs(lm) => match lm {
                            ConvoViewLastMessageRefs::MessageView(mv) => {
                                rsx! {
                                    SenderName { did: mv.sender.did.to_owned() }
                                    "{mv.text}"
                                }
                            }
                            ConvoViewLastMessageRefs::DeletedMessageView(_) => {
                                rsx! { "message has benn delete" }
                            }
                        }
                        .into(),
                        Union::Unknown(_) => None,
                    })
                    .flatten()
                    .unwrap_or_else(|| rsx! { " " });
                rsx! {
                    div { class: "border",
                        Link {
                            to: AppRoute::ConversationPage {
                                user_did: user_did.clone(),
                                conversation_id: cv.id.to_string(),
                            },
                            img {
                                class: "size-6 inline-block rounded-full",
                                src: avatar,
                            }
                            div { {lm} }
                        }
                    }
                }
            });
            rsx! {
                {convos}
            }
        } else {
            rsx! { "can not load convos" }
        };
        rsx! {
            details { open: true,
                summary {
                    Avatar { user_did: UserDid(user_did.clone()) }
                }
                {s}
            }
        }
    });

    rsx! {
        {x}
    }
}

async fn get_conversations(did: Did) -> Result<list_convos::Output, anyhow::Error> {
    let session_store = get_session_store().await?;
    let agent = session_store.get_agent(did).await?;

    let convos = agent
        .chat_bsky_convo_listConvos(
            list_convos::ParametersData {
                limit: None,
                cursor: None,
            }
            .into(),
        )
        .await?;

    Ok(convos)
}

#[component]
fn SenderName(did: Did) -> Element {
    let profile = use_resource(move || {
        to_owned![did];
        async move {
            Ok::<_, anyhow::Error>(
                get_main_agent()
                    .await?
                    .app_bsky_actor_getProfile(get_profile::ParametersData {
                        actor: did.to_owned().into(),
                    })
                    .await?,
            )
        }
    });

    if let Some(Ok(profile)) = profile.read().as_ref() {
        if let Some(display_name) = &profile.display_name {
            return rsx! { "{display_name} : " };
        }
    }

    rsx! {}
}
