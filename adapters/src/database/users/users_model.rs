use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub friends: Vec<ObjectId>,
}

impl UserModel {
    pub fn new(
        id: &Option<ObjectId>,
        username: &str,
        nickname: &str,
        password: &str,
        email: &str,
        friends: &[ObjectId],
    ) -> Self {
        Self {
            id: *id,
            username: username.to_string(),
            nickname: nickname.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            friends: friends.to_owned(),
        }
    }
}
