use bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    fmt,
    str::{self, FromStr},
};

pub trait Id<'de>:
    fmt::Display + From<&'de str> + From<ObjectId> + Into<ObjectId> + Deserialize<'de>
{
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct InparkId(String);

impl<'a> Id<'a> for InparkId {}

impl From<&str> for InparkId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for InparkId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for InparkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = bson::oid::ObjectId::deserialize(deserializer)?;
        Ok(Self(id.to_string()))
    }
}

impl From<ObjectId> for InparkId {
    fn from(value: ObjectId) -> Self {
        Self(value.to_string())
    }
}

#[allow(clippy::from_over_into)]
impl Into<ObjectId> for InparkId {
    fn into(self) -> ObjectId {
        ObjectId::from_str(&self.0).unwrap()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct UserId(String);

impl<'a> Id<'a> for UserId {}

impl From<&str> for UserId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = bson::oid::ObjectId::deserialize(deserializer)?;
        Ok(Self(id.to_string()))
    }
}

impl From<ObjectId> for UserId {
    fn from(value: ObjectId) -> Self {
        Self(value.to_string())
    }
}

#[allow(clippy::from_over_into)]
impl Into<ObjectId> for UserId {
    fn into(self) -> ObjectId {
        ObjectId::from_str(&self.0).unwrap()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ParkId(String);

impl<'a> Id<'a> for ParkId {}

impl From<&str> for ParkId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for ParkId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for ParkId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = bson::oid::ObjectId::deserialize(deserializer)?;
        Ok(Self(id.to_string()))
    }
}

impl From<ObjectId> for ParkId {
    fn from(value: ObjectId) -> Self {
        Self(value.to_string())
    }
}

#[allow(clippy::from_over_into)]
impl Into<ObjectId> for ParkId {
    fn into(self) -> ObjectId {
        ObjectId::from_str(&self.0).unwrap()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PostId(String);

impl<'a> Id<'a> for PostId {}

impl From<&str> for PostId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for PostId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for PostId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = bson::oid::ObjectId::deserialize(deserializer)?;
        Ok(Self(id.to_string()))
    }
}

impl From<ObjectId> for PostId {
    fn from(value: ObjectId) -> Self {
        Self(value.to_string())
    }
}

#[allow(clippy::from_over_into)]
impl Into<ObjectId> for PostId {
    fn into(self) -> ObjectId {
        ObjectId::from_str(&self.0).unwrap()
    }
}
