use crate::i18n::Language;

pub fn get_client_language() -> Language {
    #[cfg(target_family = "wasm")]
    {
        return web_sys::window()
            .map(|window| window.navigator().language())
            .flatten()
            .map(crate::util::language::language_str_to_enum)
            .unwrap_or(Language::En);
    }
    Language::En
}

pub fn get_client_languages() -> Vec<Language> {
    #[cfg(target_family = "wasm")]
    {
        return web_sys::window()
            .map(|window| {
                let languages = window.navigator().languages().to_vec();
                languages
                    .into_iter()
                    .filter_map(|language_jsvalue| language_jsvalue.as_string())
                    .map(language_str_to_enum)
                    .collect::<Vec<_>>()
            })
            .unwrap_or(Vec::new());
    }
    Vec::new()
}

fn language_str_to_enum(language: impl AsRef<str>) -> Language {
    let language = language.as_ref().to_ascii_lowercase();
    if language.starts_with("en") {
        Language::En
    } else if language.starts_with("ja") {
        Language::Ja
    } else if language.starts_with("zh") {
        Language::Zh
    } else {
        Language::En
    }
}
