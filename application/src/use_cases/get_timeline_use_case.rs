use super::abstract_use_case::AbstractUseCase;
use crate::repositories::{
    inpark_repository_abstract::InparkRepositoryAbstract,
    users_repository_abstract::UsersRepositoryAbstract,
};
use async_trait::async_trait;
use domain::{
    aggregates::detailed_inpark::DetailedInpark, common::error::ApiError, value_structs::id::UserId,
};

pub struct GetTimelineUseCase<'a> {
    inpark_repository: &'a dyn InparkRepositoryAbstract,
    user_repository: &'a dyn UsersRepositoryAbstract,
    user_id: &'a UserId,
}

impl<'a> GetTimelineUseCase<'a> {
    pub fn new(
        inpark_repository: &'a dyn InparkRepositoryAbstract,
        user_repository: &'a dyn UsersRepositoryAbstract,
        user_id: &'a UserId,
    ) -> Self {
        Self {
            inpark_repository,
            user_repository,
            user_id,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<DetailedInpark>> for GetTimelineUseCase<'a> {
    async fn execute(&self) -> Result<Vec<DetailedInpark>, ApiError> {
        let user = self.user_repository.get_user_by_id(self.user_id).await?;

        let detailed_inparks = self
            .inpark_repository
            .get_detailed_inparks_by_multiple_users(&user.friends)
            .await?;

        match detailed_inparks.len() {
            0 => Ok([].to_vec()),
            _ => Ok(detailed_inparks),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{
        inpark_repository_abstract::MockInparkRepositoryAbstract,
        users_repository_abstract::MockUsersRepositoryAbstract,
    };
    use bson;
    use domain::{
        entities::user_entity::{UserEntity, UserEntityWithPassword},
        value_structs::{
            date_time::DateTime,
            hashed_password::HashedPassword,
            id::{InparkId, ParkId},
        },
    };

    #[actix_rt::test]
    async fn can_get_timeline() {
        let user_id = UserId::from("test_user_id");
        let user =
            UserEntityWithPassword::new(&user_id, "", "", &HashedPassword::from(""), "", &[]);

        let bson_now = bson::DateTime::now();
        let date_time_now = DateTime::from(bson_now);

        let user_1 = UserEntity::new("user_id_1", "", "", "", &[]);
        let user_2 = UserEntity::new("user_id_2", "", "", "", &[]);

        let mut user_repository = MockUsersRepositoryAbstract::new();
        let mut inpark_repository = MockInparkRepositoryAbstract::new();

        user_repository
            .expect_get_user_by_id()
            .return_once(|_user_id| Ok(user));

        let detailed_inparks = [
            DetailedInpark::new(
                &InparkId::from("inpark_id_1"),
                &user_1,
                &ParkId::from(""),
                &date_time_now,
            ),
            DetailedInpark::new(
                &InparkId::from("inpark_id_2"),
                &user_1,
                &ParkId::from(""),
                &date_time_now,
            ),
            DetailedInpark::new(
                &InparkId::from("inpark_id_3"),
                &user_2,
                &ParkId::from(""),
                &date_time_now,
            ),
        ];

        {
            let detailed_inparks_clone = detailed_inparks.to_vec().clone();
            inpark_repository
                .expect_get_detailed_inparks_by_multiple_users()
                .return_once(|_user_ids| Ok(detailed_inparks_clone));
        }

        let get_timeline_use_case =
            GetTimelineUseCase::new(&inpark_repository, &user_repository, &user_id);

        let result = get_timeline_use_case.execute().await.unwrap();

        assert_eq!(result, detailed_inparks);
    }

    #[actix_rt::test]
    async fn return_empty_vec_if_no_inparks() {
        let user_id = UserId::from("test_user_id");
        let user =
            UserEntityWithPassword::new(&user_id, "", "", &HashedPassword::from(""), "", &[]);

        let mut user_repository = MockUsersRepositoryAbstract::new();
        let mut inpark_repository = MockInparkRepositoryAbstract::new();

        user_repository
            .expect_get_user_by_id()
            .return_once(|_user_id| Ok(user));

        inpark_repository
            .expect_get_detailed_inparks_by_multiple_users()
            .return_once(|_user_ids| Ok([].to_vec()));

        let get_timeline_use_case =
            GetTimelineUseCase::new(&inpark_repository, &user_repository, &user_id);
        let result = get_timeline_use_case.execute().await.unwrap();

        assert_eq!(result, []);
    }
}
