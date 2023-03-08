use super::abstract_use_case::AbstractUseCase;
use crate::repositories::users_repository_abstract::UsersRepositoryAbstract;
use async_trait::async_trait;
use domain::{common::error::ApiError, entities::user_entity::UserEntity};

pub struct GetPublicProfileUseCase<'a> {
    repository: &'a dyn UsersRepositoryAbstract,
    username: &'a str,
}

impl<'a> GetPublicProfileUseCase<'a> {
    pub fn new(repository: &'a dyn UsersRepositoryAbstract, username: &'a str) -> Self {
        Self {
            repository,
            username,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<UserEntity> for GetPublicProfileUseCase<'a> {
    async fn execute(&self) -> Result<UserEntity, ApiError> {
        let user = self.repository.get_user_by_username(self.username).await?;

        Ok(user.to_user_entity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::users_repository_abstract::MockUsersRepositoryAbstract;
    use domain::{
        entities::user_entity::UserEntityWithPassword,
        value_structs::{hashed_password::HashedPassword, id::UserId},
    };
    use mockall::predicate::eq;

    #[actix_rt::test]
    async fn can_return_single_user() {
        let id = "test_id";
        let user_id = UserId::from(id);
        let username = "test_username";
        let nickname = "test_nickname";
        let password = HashedPassword::from("test_password");
        let email = "test@test.com";

        let user_entity_with_password =
            UserEntityWithPassword::new(&user_id, username, nickname, &password, email, &[]);

        let mut user_repository = MockUsersRepositoryAbstract::new();

        {
            let user_entity_with_password_clone = user_entity_with_password.clone();
            user_repository
                .expect_get_user_by_username()
                .with(eq(username))
                .times(1)
                .return_once(move |_user_id| Ok(user_entity_with_password_clone.clone()));
        }

        let get_public_profile_usecase = GetPublicProfileUseCase::new(&user_repository, &username);
        let result_user_entity = get_public_profile_usecase.execute().await.unwrap();
        let user_entity = user_entity_with_password.to_user_entity();

        assert_eq!(user_entity, result_user_entity);
    }
}
