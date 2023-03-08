use super::abstract_use_case::AbstractUseCase;
use crate::repositories::parks_repository_abstract::ParksRepositoryAbstract;
use async_trait::async_trait;
use domain::{common::error::ApiError, entities::park_entity::ParkEntity};

pub struct GetAllParksUseCase<'a> {
    parks_repository: &'a dyn ParksRepositoryAbstract,
}

impl<'a> GetAllParksUseCase<'a> {
    pub fn new(parks_repository: &'a dyn ParksRepositoryAbstract) -> Self {
        Self { parks_repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<ParkEntity>> for GetAllParksUseCase<'a> {
    async fn execute(&self) -> Result<Vec<ParkEntity>, ApiError> {
        let parks = self.parks_repository.get_all_parks().await?;

        Ok(parks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::parks_repository_abstract::MockParksRepositoryAbstract;
    use domain::value_structs::id::ParkId;

    #[actix_rt::test]
    async fn can_get_all_parks() {
        let park_entities = vec![
            ParkEntity::new(&ParkId::from("park_id_1"), "park_name_1", "park_name_1_ja"),
            ParkEntity::new(&ParkId::from("park_id_2"), "park_name_2", "park_name_2_ja"),
        ];

        let mut parks_repository = MockParksRepositoryAbstract::new();
        {
            let park_entity_clone = park_entities.clone();
            parks_repository
                .expect_get_all_parks()
                .return_once(|| Ok(park_entity_clone));
        }

        let get_all_parks_use_case = GetAllParksUseCase::new(&parks_repository);

        let result = get_all_parks_use_case.execute().await.unwrap();

        assert_eq!(result, park_entities);
    }
}
