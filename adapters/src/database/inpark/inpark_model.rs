use bson::DateTime;
use domain::value_structs::id::{ParkId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InparkModel {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub park_id: ParkId,
    pub user_id: UserId,
    pub date_time: DateTime,
}
