use crate::{
    entities::user_entity::UserEntity,
    value_structs::{
        date_time::DateTime,
        id::{InparkId, ParkId},
    },
};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DetailedInpark {
    #[serde(rename = "_id")]
    pub id: InparkId,
    pub user: UserEntity,
    pub park_id: ParkId,
    pub date_time: DateTime,
}

impl DetailedInpark {
    pub fn new(id: &InparkId, user: &UserEntity, park_id: &ParkId, date_time: &DateTime) -> Self {
        Self {
            id: id.to_owned(),
            user: user.to_owned(),
            park_id: park_id.to_owned(),
            date_time: date_time.to_owned(),
        }
    }
}
