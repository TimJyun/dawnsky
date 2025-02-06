use crate::i18n::cn::CN_TEXT;
use crate::i18n::en::EN_TEXT;
use crate::i18n::ja::JA_TEXT;
use crate::i18n::Language;
use crate::util::language::get_client_language;
use dioxus::prelude::{GlobalMemo, Signal};

pub static TEXT: GlobalMemo<Text> = Signal::global_memo(|| {
    let language = get_client_language();
    match language {
        Language::Zh => CN_TEXT,
        Language::En => EN_TEXT,
        Language::Ja => JA_TEXT,
    }
});

#[derive(PartialEq)]
pub struct Text {
    pub bottom_to_bookshelf_page: &'static str,
    pub bottom_to_discovery_page: &'static str,
    pub bottom_to_message_page: &'static str,
    pub bottom_to_my_page: &'static str,
}
