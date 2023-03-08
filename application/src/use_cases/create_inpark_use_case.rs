use super::abstract_use_case::AbstractUseCase;
use crate::repositories::inpark_repository_abstract::InparkRepositoryAbstract;
use async_trait::async_trait;
use domain::{
    common::error::ApiError,
    value_structs::{
        date_time::DateTime,
        id::{InparkId, ParkId, UserId},
    },
};

pub struct CreateSingleInparkUseCase<'a> {
    repository: &'a dyn InparkRepositoryAbstract,
    park_id: &'a ParkId,
    user_id: &'a UserId,
    date: &'a DateTime,
}

impl<'a> CreateSingleInparkUseCase<'a> {
    pub fn new(
        repository: &'a dyn InparkRepositoryAbstract,
        park_id: &'a ParkId,
        user_id: &'a UserId,
        date: &'a DateTime,
    ) -> Self {
        Self {
            repository,
            park_id,
            user_id,
            date,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<InparkId> for CreateSingleInparkUseCase<'a> {
    async fn execute(&self) -> Result<InparkId, ApiError> {
        let id = self
            .repository
            .create_inpark(self.park_id, self.user_id, self.date)
            .await?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        repositories::inpark_repository_abstract::MockInparkRepositoryAbstract,
        use_cases::abstract_use_case::AbstractUseCase,
    };
    use bson;
    use domain::value_structs::date_time;

    #[actix_rt::test]
    async fn can_create_inpark() {
        let park_id = ParkId::from("park_id");
        let user_id = UserId::from("user_id");
        let now = bson::DateTime::now();
        let date = date_time::DateTime::from(now);
        let inpark_id = InparkId::from("inpark_id");

        let mut inpark_repository = MockInparkRepositoryAbstract::new();

        {
            let inpark_id_clone = inpark_id.clone();
            inpark_repository
                .expect_create_inpark()
                .return_once(move |_park_id, _user_id, _date| Ok(inpark_id_clone));
        }

        let create_inpark =
            CreateSingleInparkUseCase::new(&inpark_repository, &park_id, &user_id, &date);

        let result = create_inpark.execute().await.unwrap();

        assert_eq!(result, inpark_id);
    }
}
