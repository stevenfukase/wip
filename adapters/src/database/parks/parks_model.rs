use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParksModel {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: String,
    pub name_ja: String,
}
