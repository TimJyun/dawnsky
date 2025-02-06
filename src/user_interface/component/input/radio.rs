use crate::imgs::{ACTIVE_STATE_ICON_50_50, SELECT_ICON_50_50};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[derive(Props, Clone, Debug, PartialEq)]
pub struct RadioProps {
    selected: bool,
}

pub fn Radio(props: RadioProps) -> Element {
    if props.selected {
        rsx! {
            img {
                class: "size-5 select-none inline-block",
                src: SELECT_ICON_50_50,
            }
        }
    } else {
        rsx! {
            img {
                class: "size-5 select-none inline-block",
                src: ACTIVE_STATE_ICON_50_50,
            }
        }
    }
}
