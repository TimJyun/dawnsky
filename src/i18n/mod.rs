use serde::{Deserialize, Serialize};

pub(super) mod cn;
pub(super) mod en;
pub(super) mod ja;

#[derive(Hash, Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Language {
    Zh = 0,
    Ja = 1,
    #[default]
    En = 2,
}
