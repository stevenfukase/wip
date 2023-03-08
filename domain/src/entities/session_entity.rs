use crate::{
    enums::role::Role,
    value_structs::{id::UserId, token::Token},
};
use serde::Serialize;

#[non_exhaustive]
#[derive(Clone, Serialize, Debug, PartialEq, Eq)]
pub struct SessionEntity {
    pub user_id: UserId,
    pub roles: Vec<Role>,
    pub token: Token,
    pub exp: i64,
}

impl SessionEntity {
    pub fn new(user_id: &UserId, roles: &[Role], token: &Token, exp: i64) -> Self {
        Self {
            user_id: user_id.to_owned(),
            roles: roles.to_vec(),
            token: token.to_owned(),
            exp: exp.to_owned(),
        }
    }
}
