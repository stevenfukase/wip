use crate::value_structs::{hashed_password::HashedPassword, id::UserId};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserEntity {
    #[serde(rename(deserialize = "_id"))]
    pub id: UserId,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub friends: Vec<UserId>,
}

impl UserEntity {
    pub fn new(id: &str, username: &str, nickname: &str, email: &str, friends: &[UserId]) -> Self {
        Self {
            id: UserId::from(id),
            username: username.to_string(),
            nickname: nickname.to_string(),
            email: email.to_string(),
            friends: friends.to_vec(),
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEntityWithPassword {
    pub id: UserId,
    pub username: String,
    pub nickname: String,
    pub password: HashedPassword,
    pub email: String,
    pub friends: Vec<UserId>,
}

impl UserEntityWithPassword {
    pub fn new(
        id: &UserId,
        username: &str,
        nickname: &str,
        password: &HashedPassword,
        email: &str,
        friends: &[UserId],
    ) -> Self {
        Self {
            id: id.to_owned(),
            username: username.to_string(),
            nickname: nickname.to_string(),
            password: password.to_owned(),
            email: email.to_string(),
            friends: friends.to_vec(),
        }
    }

    pub fn to_user_entity(&self) -> UserEntity {
        UserEntity {
            id: self.id.to_owned(),
            username: self.username.to_owned(),
            nickname: self.nickname.to_owned(),
            email: self.email.to_owned(),
            friends: self.friends.to_owned(),
        }
    }
}
