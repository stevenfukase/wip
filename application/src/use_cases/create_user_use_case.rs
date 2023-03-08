use super::abstract_use_case::AbstractUseCase;
use crate::repositories::{
    auth_repository_abstract::AuthRepositoryAbstract,
    users_repository_abstract::UsersRepositoryAbstract,
};
use async_trait::async_trait;
use domain::{
    common::error::ApiError, entities::user_entity::UserEntity,
    enums::create_user_error::CreateUserError,
};

#[non_exhaustive]
pub struct CreateUserUseCase<'a> {
    users_repository: &'a dyn UsersRepositoryAbstract,
    auth_repository: &'a dyn AuthRepositoryAbstract,
    username: String,
    nickname: String,
    email: String,
    password: String,
}

impl<'a> CreateUserUseCase<'a> {
    pub fn new(
        users_repository: &'a dyn UsersRepositoryAbstract,
        auth_repository: &'a dyn AuthRepositoryAbstract,
        username: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Self {
        Self {
            users_repository,
            auth_repository,
            username: username.to_string(),
            nickname: nickname.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for CreateUserUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user_with_same_username_or_email = self
            .users_repository
            .get_user_with_email_and_username(&self.username, &self.email)
            .await?;

        if let Some(user) = user_with_same_username_or_email {
            if user.email == self.email {
                return Err(CreateUserError::EmailExists.into());
            }

            if user.username == self.username {
                return Err(CreateUserError::UsernameExists.into());
            }
        }

        let hashed_password = self.auth_repository.hash_password(&self.password)?;

        let user_entity = self
            .users_repository
            .insert_new_user(
                &self.username,
                &self.nickname,
                &self.email,
                &hashed_password.to_string(),
            )
            .await?;

        Ok(user_entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        auth_repository_abstract::MockAuthRepositoryAbstract,
        users_repository_abstract::MockUsersRepositoryAbstract,
    };
    use domain::value_structs::hashed_password::HashedPassword;

    #[actix_rt::test]
    async fn can_create_new_user() {
        let username = "stevenfukase";
        let password = "encrypted";
        let nickname = "nickname";
        let email = "test@email.com";
        let id = "stevenfukase";

        let user_entity = UserEntity::new(id, username, nickname, email, &[]);

        let mut users_repository = MockUsersRepositoryAbstract::new();
        let mut auth_repository = MockAuthRepositoryAbstract::new();

        users_repository
            .expect_get_user_with_email_and_username()
            .return_once(|_username, _password| Ok(None));

        {
            let user_entity_clone = user_entity.clone();
            users_repository
                .expect_insert_new_user()
                .return_once(move |_username, _nickname, _email, _password| Ok(user_entity_clone));
        }

        auth_repository
            .expect_hash_password()
            .return_once(|password_str| Ok(HashedPassword::from(password_str)));

        let create_user_usecase = CreateUserUseCase::new(
            &users_repository,
            &auth_repository,
            username,
            nickname,
            email,
            password,
        );

        let result = create_user_usecase.execute().await;

        assert!(result.is_ok());
        let result_user_entity = result.unwrap();
        assert_eq!(result_user_entity, user_entity);
    }
}
