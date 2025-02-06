use atrium_api::types::string::Did;
use derive_more::with_trait::{Display, FromStr};
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct UserDid(pub Did);

impl Display for UserDid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl FromStr for UserDid {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserDid(Did::from_str(s)?))
    }
}

impl PartialOrd for UserDid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.as_str().partial_cmp(other.0.as_str())
    }
}

impl Ord for UserDid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.as_str().cmp(other.0.as_str())
    }
}
