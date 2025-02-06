use crate::imgs::{CHECKBOX_CHECKED_ICON_50_50, CHECKBOX_ICON_50_50};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Props, Clone, Debug, PartialEq)]
pub struct CheckBoxProps {
    checked: bool,
    onclick: Option<EventHandler<MouseEvent>>,
}

pub fn CheckBox(props: CheckBoxProps) -> Element {
    if props.checked {
        if let Some(onclick) = props.onclick {
            rsx! {
                img {
                    class: "size-5 select-none inline-block",
                    src: CHECKBOX_CHECKED_ICON_50_50,
                    onclick: move |evt| { onclick.call(evt) },
                }
            }
        } else {
            rsx! {
                img {
                    class: "size-5 select-none inline-block",
                    src: CHECKBOX_CHECKED_ICON_50_50,
                }
            }
        }
    } else {
        if let Some(onclick) = props.onclick {
            rsx! {
                img {
                    class: "size-5 select-none inline-block",
                    src: CHECKBOX_ICON_50_50,
                    onclick: move |evt| { onclick.call(evt) },
                }
            }
        } else {
            rsx! {
                img {
                    class: "size-5 select-none inline-block",
                    src: CHECKBOX_ICON_50_50,
                }
            }
        }
    }
}
