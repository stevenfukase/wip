use async_trait::async_trait;
use domain::{
    entities::claims_entity::Claims,
    enums::role::Role,
    value_structs::{hashed_password::HashedPassword, id::UserId, token::Token},
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait AuthRepositoryAbstract {
    fn create_token(
        &self,
        uid: &UserId,
        permissions: &[Role],
    ) -> Result<(Token, i64), Box<dyn Error>>;

    async fn decode_token(&self, token: &Token) -> Result<Claims, Box<dyn Error>>;

    fn hash_password(&self, password: &str) -> Result<HashedPassword, Box<dyn Error>>;

    fn hashes_match(
        &self,
        hashed_password: &HashedPassword,
        string: &str,
    ) -> Result<bool, Box<dyn Error>>;
}
