use bson;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateTime(bson::DateTime);

impl DateTime {
    pub fn to_bson(&self) -> bson::DateTime {
        self.0
    }
}

impl From<bson::DateTime> for DateTime {
    fn from(value: bson::DateTime) -> Self {
        Self(value)
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
