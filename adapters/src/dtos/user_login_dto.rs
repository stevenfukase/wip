use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginDto {
    pub username: String,
    pub password: String,
}
