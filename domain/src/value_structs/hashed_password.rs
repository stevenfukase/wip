use core::fmt;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HashedPassword(String);

impl From<&str> for HashedPassword {
    fn from(value: &str) -> Self {
        HashedPassword(value.to_string())
    }
}

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
