use super::{parks_db_mapper::ParksDbMapper, parks_model::ParksModel};
use crate::database::shared::db_connection::DbConnection;
use application::{
    mappers::db_mapper::DbMapper, repositories::parks_repository_abstract::ParksRepositoryAbstract,
};
use async_trait::async_trait;
use domain::entities::park_entity::ParkEntity;
use futures::stream::TryStreamExt;
use mongodb::options::FindOptions;
use std::sync::Arc;

pub struct ParksRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl ParksRepositoryAbstract for ParksRepository {
    async fn get_all_parks(&self) -> Result<Vec<ParkEntity>, Box<dyn std::error::Error>> {
        let db = self.db_connection.get_pool().await;
        let collection = db.collection::<ParksModel>("parks");
        let find_options = FindOptions::builder().build();
        let cursor = collection.find(None, find_options).await?;
        let results = cursor.try_collect::<Vec<ParksModel>>().await?;
        let entities = results
            .into_iter()
            .map(|model| ParksDbMapper::to_entity(&model))
            .collect();

        Ok(entities)
    }
}
