use crate::value_structs::{
    date_time::{self, DateTime},
    id::{InparkId, ParkId, UserId},
};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize)]
pub struct InparkEntity {
    pub id: InparkId,
    pub user_id: UserId,
    pub park_id: ParkId,
    pub date_time: date_time::DateTime,
}

impl InparkEntity {
    pub fn new(id: &InparkId, user_id: &UserId, park_id: &ParkId, date_time: &DateTime) -> Self {
        Self {
            id: id.to_owned(),
            user_id: user_id.to_owned(),
            park_id: park_id.to_owned(),
            date_time: date_time.to_owned(),
        }
    }
}
