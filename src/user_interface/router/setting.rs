use crate::util::sleep::sleep;
use dioxus::prelude::*;

use std::cell::Ref;

use crate::user_interface::client_util::go_back_or_replace_to_index;

use dioxus::html::completions::CompleteWithBraces::text;
use dioxus_html::FileEngine;
use opendal::Operator;

use anyhow::anyhow;
use atrium_api::app::bsky::actor::{get_profile, Profile};
use atrium_api::client::com;
use atrium_api::com::atproto::repo::{get_record, put_record};
use atrium_api::com::atproto::sync::get_blob;
use atrium_api::record::KnownRecord;
use atrium_api::types::string::{Cid, Did, Handle, Nsid};
use atrium_api::types::{
    BlobRef, Collection, TryFromUnknown, TryIntoUnknown, TypedBlobRef, Unknown,
};
use chrono::NaiveDate;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;

use crate::pds::agent::{get_main_agent, get_main_session_did};

use crate::i18n::Language;
use crate::pds::noauth::PublicAtAgent;
use crate::pds::store::SessionStore;
use crate::pds::user_did::UserDid;
use crate::storage::session_store::get_session_store;

use crate::user_interface::app::{refresh_app, reload_page};
use crate::user_interface::component::avatar::Avatar;
use crate::user_interface::component::confirm_box::confirm;
use crate::user_interface::router::AppRoute;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

pub mod password;
mod profile;

pub fn SettingPage() -> Element {
    let mut busying = use_signal(|| false);

    let mut handle_with_main_profile_res = use_resource(|| async {
        let agent = get_main_agent().await?;
        let session = agent.get_session().await?;
        Ok::<_, anyhow::Error>((
            session.handle.to_string(),
            session.did.to_string(),
            agent.ext_get_my_profile().await?,
        ))
    });

    let handle_with_main_profile = handle_with_main_profile_res.suspend()?;

    let main_profile_node = if let Ok((handle, did, main_profile)) =
        handle_with_main_profile.read().as_ref()
    {
        let avatar = main_profile
            .avatar
            .as_ref()
            .map(|b| {
                let cid = match b {
                    BlobRef::Typed(TypedBlobRef::Blob(b)) => b.r#ref.0.to_string(),
                    BlobRef::Untyped(b) => b.cid.to_string(),
                };

                let url = PublicAtAgent::from_handle(handle)
                    .com_atproto_sync_getBlob(get_blob::ParametersData {
                        cid: Cid::from_str(&cid).ok()?,
                        did: Did::from_str(&did).ok()?,
                    })
                    .ok()?;
                Some(url.0)
            })
            .flatten()
            .unwrap_or_default();

        rsx! {
            div {
                label { class: "inline-block",
                    img { class: "size-16", src: avatar, alt: "user photo" }
                    input {
                        class: "hidden",
                        r#type: "file",
                        accept: ".png,.bmp,.jpg,.jpeg,.webp",
                        multiple: false,
                        disabled: *busying.read(),
                        onchange: move |evt| {
                            busying.set(true);
                            async move {
                                if let Some(file_engine) = &evt.files() {
                                    if let Err(err) = upload_user_photo_client(file_engine).await {
                                        debug!("upload user photo error : {err}");
                                    } else {
                                        debug!("upload user photo success");
                                        refresh_app();
                                    }
                                }
                                busying.set(false);
                            }
                        },
                    }
                }
                label { class: "inline-block",
                    div { "display name" }
                    div {
                        input {
                            disabled: *busying.read(),
                            value: main_profile.display_name.clone().unwrap_or_default(),
                            onchange: move |evt| {
                                busying.set(true);
                                let new_display_name = evt.value();
                                async move {
                                    if let Err(err) = set_display_name(new_display_name).await {
                                        debug!("change display name error : {err}");
                                    } else {
                                        debug!("change display name success");
                                        refresh_app();
                                    }
                                    busying.set(false);
                                }
                            },
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        {main_profile_node}
        details { open: false,
            summary { "switch account" }
            Accounts {}
        }
    }
}

pub async fn set_display_name(name: String) -> Result<(), anyhow::Error> {
    let agent = get_main_agent().await?;

    let mut profile = agent.ext_get_my_profile().await?;

    profile.display_name = Some(name);

    agent.ext_set_my_profile(profile).await?;
    Ok(())
}

pub async fn upload_user_photo_client(files: &Arc<dyn FileEngine>) -> Result<(), anyhow::Error> {
    let file_names = files.files();
    let file_name = file_names.first().ok_or(anyhow!("no filename"))?;
    let file = files.read_file(file_name).await.ok_or(anyhow!("no file"))?;

    let agent = get_main_agent().await?;
    let cid = agent.com_atproto_repo_uploadBlob(file, "image/*").await?;
    let mut profile = agent.ext_get_my_profile().await?;

    profile.avatar = Some(cid.blob.clone());

    agent.ext_set_my_profile(profile).await?;

    Ok(())
}

pub async fn my_profile() -> Option<()> {
    let agent = get_main_agent().await.ok()?;

    agent
        .app_bsky_actor_getProfile(get_profile::ParametersData {
            actor: agent.did.clone().into(),
        })
        .await
        .ok()?;

    Some(())
}

fn Accounts() -> Element {
    let mut accounts_res = use_resource(|| async {
        let session_store = get_session_store().await?;
        let dids = session_store.list_sessions().await?;
        let mut sessions = Vec::with_capacity(dids.len());
        for did in dids.into_iter() {
            sessions.push(session_store.get_session(did).await?);
        }
        Ok::<_, anyhow::Error>(sessions)
    });

    let main_session_did = use_signal(get_main_session_did);

    match accounts_res.suspend()?.read().as_ref() {
        Ok(sessions) => {
            let accounts = sessions.iter().cloned().map(move |s| {
                let set_main_session = if Some(&s.did) == main_session_did.peek().as_ref().ok() {
                    rsx! {
                        span { "★" }
                    }
                } else {
                    rsx! {
                        span {
                            onclick: {
                                let did = s.did.clone();
                                move |_| {
                                    let _ = LocalStorage::set(crate::pds::agent::MAIN_SESSION_DID, &did);
                                    refresh_app();
                                }
                            },
                            "☆"
                        }
                    }
                };
                rsx! {
                    div {
                        Avatar { user_did: UserDid(s.did.clone()) }
                        span {
                            onclick: move |_| {
                                let did = s.did.clone();
                                async move {
                                    if confirm(["do you want to remove the account from you device"]).await {
                                        let session_store = get_session_store().await.unwrap();
                                        session_store.remove_session(did.clone()).await.unwrap();
                                        accounts_res.restart();
                                    }
                                    Ok(())
                                }
                            },
                            {set_main_session}
                            "x"
                        }
                    }
                }
            });
            rsx! {
                {accounts}
                Link { to: AppRoute::LoginPage {}, "add account" }
            }
        }
        Err(err) => {
            rsx! { "error : {err}" }
        }
    }
}
