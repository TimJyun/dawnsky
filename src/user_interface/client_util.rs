use crate::i18n::Language;

use crate::user_interface::router::AppRoute;
use dioxus::prelude::Navigator;

pub fn get_user_language() -> Vec<Language> {
    vec![Language::Zh]
}

pub fn go_back_or_replace_to_index(nav: Navigator) {
    if nav.can_go_back() {
        nav.go_back();
    } else {
        nav.replace(AppRoute::SocialPage {});
    }
}
