use async_trait::async_trait;
use domain::{
    entities::user_entity::{UserEntity, UserEntityWithPassword},
    value_structs::id::UserId,
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait UsersRepositoryAbstract {
    async fn get_user_by_id(
        &self,
        user_id: &UserId,
    ) -> Result<UserEntityWithPassword, Box<dyn Error>>;

    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<UserEntityWithPassword, Box<dyn Error>>;

    async fn insert_new_user(
        &self,
        username: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserEntity, Box<dyn Error>>;

    async fn get_user_with_email_and_username(
        &self,
        username: &str,
        email: &str,
    ) -> Result<Option<UserEntity>, Box<dyn Error>>;
}
