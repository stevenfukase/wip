use bson::Bson;
use serde::{Deserialize, Serialize};
use std::fmt;

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ImageUrl(String);

impl fmt::Display for ImageUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ImageUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<ImageUrl> for Bson {
    fn from(image_url: ImageUrl) -> Self {
        Bson::String(image_url.0)
    }
}
