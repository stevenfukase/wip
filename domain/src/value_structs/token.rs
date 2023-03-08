use serde::{Deserialize, Serialize};
use std::fmt;

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token(String);

impl Token {
    pub fn new(token: &str) -> Self {
        Self(token.to_string())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
