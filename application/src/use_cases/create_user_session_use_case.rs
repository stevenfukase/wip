use super::abstract_use_case::AbstractUseCase;
use crate::repositories::{
    auth_repository_abstract::AuthRepositoryAbstract,
    users_repository_abstract::UsersRepositoryAbstract,
};
use async_trait::async_trait;
use domain::{common::error::ApiError, entities::session_entity::SessionEntity, enums::role::Role};
use std::io;

pub struct CreateUserSessionUseCase<'a> {
    users_repository: &'a dyn UsersRepositoryAbstract,
    auth_repository: &'a dyn AuthRepositoryAbstract,
    username: &'a str,
    password: &'a str,
}

impl<'a> CreateUserSessionUseCase<'a> {
    pub fn new(
        users_repository: &'a dyn UsersRepositoryAbstract,
        auth_repository: &'a dyn AuthRepositoryAbstract,
        username: &'a str,
        password: &'a str,
    ) -> Self {
        Self {
            users_repository,
            auth_repository,
            username,
            password,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SessionEntity> for CreateUserSessionUseCase<'a> {
    async fn execute(&self) -> Result<SessionEntity, ApiError> {
        let user = self
            .users_repository
            .get_user_by_username(self.username)
            .await?;

        let password_matches = self
            .auth_repository
            .hashes_match(&user.password, self.password);

        if password_matches.is_err() || !password_matches.unwrap() {
            let permission_error: Box<dyn std::error::Error> =
                Box::new(io::Error::from(io::ErrorKind::PermissionDenied));
            return Err(ApiError::from(permission_error));
        };

        let roles = [Role::USER];
        let (token, exp) = self.auth_repository.create_token(&user.id, &roles)?;

        Ok(SessionEntity::new(&user.id, &roles, &token, exp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        auth_repository_abstract::MockAuthRepositoryAbstract,
        users_repository_abstract::MockUsersRepositoryAbstract,
    };
    use domain::{
        entities::user_entity::UserEntityWithPassword,
        value_structs::{hashed_password::HashedPassword, id::UserId, token::Token},
    };
    use std::io::{Error, ErrorKind};

    #[actix_rt::test]
    async fn returns_error_if_users_repo_error() {
        let username = "username1";
        let password = "test_password";

        let mut users_repository = MockUsersRepositoryAbstract::new();
        let auth_repository = MockAuthRepositoryAbstract::new();

        users_repository
            .expect_get_user_by_username()
            .return_once(|_username| Err(Box::new(Error::new(ErrorKind::Other, "Some Error"))));

        let get_public_profile_usecase = CreateUserSessionUseCase::new(
            &users_repository,
            &auth_repository,
            &username,
            &password,
        );

        let data = get_public_profile_usecase.execute().await;

        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("API Error", result.message);
    }

    #[actix_rt::test]
    async fn can_create_user_session() {
        let username = "stevenfukase";
        let password = "encrypted";
        let hashed_password = HashedPassword::from(password);
        let nickname = "nickname";
        let email = "test@email.com";
        let user_id = UserId::from("stevenfukase");
        let roles = [Role::USER];
        let token = Token::new("test_token");
        let exp = 1676940772;

        let mut users_repository = MockUsersRepositoryAbstract::new();
        let mut auth_repository = MockAuthRepositoryAbstract::new();

        {
            let user_id_clone = user_id.clone();
            let hashed_password_clone = hashed_password.clone();
            users_repository
                .expect_get_user_by_username()
                .return_once(move |_username| {
                    let user = UserEntityWithPassword::new(
                        &user_id_clone,
                        username,
                        nickname,
                        &hashed_password_clone,
                        email,
                        &[],
                    );
                    Ok(user)
                });
        }

        auth_repository
            .expect_hashes_match()
            .return_once(|_hashed_password, _string| Ok(true));

        {
            let token_clone = token.clone();
            let expiry_clone = exp.clone();
            auth_repository
                .expect_create_token()
                .return_once(move |_user_id, _roles| Ok((token_clone, expiry_clone)));
        }

        let create_user_session_use_case = CreateUserSessionUseCase::new(
            &users_repository,
            &auth_repository,
            &username,
            &password,
        );

        let result = create_user_session_use_case.execute().await.unwrap();

        let session_entity = SessionEntity::new(&user_id, &roles, &token, exp);

        assert_eq!(session_entity, result);
    }
}
