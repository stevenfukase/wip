use serde::{Deserialize, Serialize};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Hashtag(String);

impl From<&str> for Hashtag {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Hashtag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
