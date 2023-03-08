use bson::serde_helpers::bson_datetime_as_rfc3339_string;
use domain::value_structs::id::ParkId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateInparkDto {
    pub park_id: ParkId,
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    pub date_time: bson::DateTime,
}
