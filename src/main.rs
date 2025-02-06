#![allow(non_snake_case)]

mod error;
mod i18n;
mod imgs;
mod pds;
pub mod storage;
mod text;
mod user_interface;
mod util;

use crate::user_interface::app::app;

fn main() {
    dioxus::launch(app);
}
