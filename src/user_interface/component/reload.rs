use crate::user_interface::client_util::get_user_language;

use crate::imgs::DELETE_TRASH_ICON_50_50;
use crate::imgs::PEN_ICON_50_50;

use dioxus::core_macro::component;
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;

use tracing::debug;

use crate::user_interface::router::AppRoute;
use chrono::{DateTime, FixedOffset};
use dioxus::dioxus_core::internal::generational_box::GenerationalRef;

use crate::i18n::Language;
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

use std::cell::Ref;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct ReloadProps<T: Restartable + 'static + PartialEq> {
    task: T,
}

pub fn Reload<T: Restartable + 'static + PartialEq>(props: ReloadProps<T>) -> Element {
    let mut task = props.task;
    rsx! {
        span { class: "size-full flex justify-center items-center",
            span {
                onclick: move |_| {
                    task.restart();
                },
                "reload"
            }
        }
    }
}

trait Restartable {
    fn restart(&mut self);
}

impl Restartable for UseFuture {
    fn restart(&mut self) {
        self.restart()
    }
}

impl<T> Restartable for Resource<T> {
    fn restart(&mut self) {
        self.restart();
    }
}
