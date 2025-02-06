use std::collections::BTreeMap;

pub fn is_kanji(c: char) -> bool {
    let c = c as u32;

    //A
    (c >= 0x3400 && c <= 0x4DBF) ||
        //中日韩统一表意文字
        (c >= 0x4E00 && c <= 0x9FFF) ||
        //cjk兼容
        (c >= 0xF900 && c <= 0xFAFF) ||
        //B
        (c >= 0x20000 && c <= 0x2A6DF) ||
        //C,D,E,F,I
        (c >= 0x2A700 && c <= 0x2EE5F) ||
        //cjk兼容扩充
        (c >= 0x2F800 && c <= 0x2FA1F) ||
        //G,H
        (c >= 0x30000 && c <= 0x323AF)
}

pub fn is_kana(c: char) -> bool {
    is_hiragana(c) || is_katakana(c)
}

pub fn is_hiragana(c: char) -> bool {
    HIRAGANA.contains(c)
}
pub fn is_katakana(c: char) -> bool {
    KATAKANA.contains(c) || c == KATAKANA_MACRON
}

pub const KATAKANA_A: &str = "アカガサザタダナハバパマャヤラワ";
pub const KATAKANA_I: &str = "イキギシジチヂニヒビピミリ";
pub const KATAKANA_U: &str = "ウクグスズッツヅヌフブプムュユル";
pub const KATAKANA_E: &str = "エケゲセゼテデネヘベペメレ";
pub const KATAKANA_O: &str = "オコゴソゾトドノホボポモョヨロヲ";

pub const KATAKANA_K: &str = "カキクケコ";
pub const KATAKANA_S: &str = "サシスセソ";
pub const KATAKANA_T: &str = "タチツテト";
pub const KATAKANA_H: &str = "ハヒフヘホ";

pub const HIRAGANA_K: &str = "かきくけこ";
pub const HIRAGANA_G: &str = "がぎぐげご";

pub const HIRAGANA_S: &str = "さしすせそ";
pub const HIRAGANA_Z: &str = "ざじずぜぞ";

pub const HIRAGANA_T: &str = "たちつてと";
pub const HIRAGANA_D: &str = "だぢづでど";

pub const HIRAGANA_H: &str = "はひふへほ";
pub const HIRAGANA_B: &str = "ばびぶべぼ";
pub const HIRAGANA_P: &str = "ぱぴぷぺぽ";

const HIRAGANA: &str = "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろわをん";
const KATAKANA: &str = "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロワヲン";
const KATAKANA_MACRON: char = 'ー';

pub const KANJI_REPEAT: char = '々';

pub fn is_start_with<T: Eq>(vec: &Vec<T>, p: &Vec<T>) -> bool {
    for i in 0..p.len() {
        if vec.get(i) != p.get(i) {
            return false;
        }
    }
    true
}

pub type Word = String;

pub fn separate_words(str: impl AsRef<str>) -> BTreeMap<Word, i32> {
    let mut result = Vec::new();
    let mut word: Vec<char> = Vec::new();

    for c in str.as_ref().chars() {
        if c.is_ascii_digit() {
            if let Some(false) = word.first().map(|c| c.is_ascii_digit()) {
                if word.len() > 0 {
                    result.push(word.iter().collect::<String>());
                    word.clear();
                }
            }
            word.push(c);
        } else if is_hiragana(c) {
            if let Some(false) = word.first().map(|c| is_hiragana(*c)) {
                if word.len() > 0 {
                    result.push(word.iter().collect::<String>());
                    word.clear();
                }
            }
            word.push(c);
        } else if is_katakana(c) {
            if let Some(false) = word.first().map(|c| is_katakana(*c)) {
                if word.len() > 0 {
                    result.push(word.iter().collect::<String>());
                    word.clear();
                }
            }
            word.push(c);
        } else if c.is_ascii_alphabetic() {
            if let Some(false) = word.first().map(|c| c.is_ascii_alphabetic()) {
                if word.len() > 0 {
                    result.push(word.iter().collect::<String>());
                    word.clear();
                }
            }
            word.push(c.to_ascii_lowercase());
        } else {
            if word.len() > 0 {
                result.push(word.iter().collect::<String>());
                word.clear();
            }
            if is_kanji(c) {
                word.push(c);
            }
        }
    }

    if word.len() > 0 {
        result.push(word.iter().collect::<String>());
        word.clear();
    }

    let mut btree_map = BTreeMap::new();

    for word in result {
        let mut count = btree_map.entry(word).or_insert(0);
        *count = *count + 1;
    }

    btree_map
}

pub fn is_legal_username(username: impl AsRef<str>) -> bool {
    username
        .as_ref()
        .chars()
        .all(|c| (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9'))
}
