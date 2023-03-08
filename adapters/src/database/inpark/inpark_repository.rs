use super::inpark_model::InparkModel;
use crate::database::shared::db_connection::DbConnection;
use application::repositories::inpark_repository_abstract::InparkRepositoryAbstract;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, Document};
use domain::{
    aggregates::detailed_inpark::DetailedInpark,
    value_structs::{
        date_time::DateTime,
        id::{InparkId, ParkId, UserId},
    },
};
use futures::TryStreamExt;
use std::{error::Error, sync::Arc};

pub struct InparkRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl InparkRepositoryAbstract for InparkRepository {
    async fn create_inpark(
        &self,
        park_id: &ParkId,
        user_id: &UserId,
        date: &DateTime,
    ) -> Result<InparkId, Box<dyn Error>> {
        let new_doc = doc! {"parkId": park_id.to_string(), "date": date.to_string(), "userId": user_id.to_string()};

        let db = self.db_connection.get_pool().await;
        let result = db
            .collection("inpark")
            .insert_one(new_doc.clone(), None)
            .await?;

        let inpark_id = result.inserted_id.as_object_id();

        if inpark_id.is_none() {
            return Err("Inpark not inserted".into());
        }

        Ok(InparkId::from(inpark_id.unwrap()))
    }

    async fn get_detailed_inparks_by_multiple_users(
        &self,
        user_ids: &[UserId],
    ) -> Result<Vec<DetailedInpark>, Box<dyn Error>> {
        let user_object_ids = user_ids
            .iter()
            .map(|user_id| user_id.clone().into())
            .collect::<Vec<ObjectId>>();

        let filter = doc! {
            "$match": {
                "user_id": {"$in": user_object_ids}
            }
        };

        let sort = doc! {
            "$sort": {
                "date_time": 1
            }
        };

        let join_user = doc! {
            "$lookup": {
                "from": "users",
                "localField": "user_id",
                "foreignField": "_id",
                "as": "user"
            }
        };

        let select_first_user_and_remove_user_id = doc! {
            "$project": {
                "_id": 1,
                "park_id": 1,
                "date_time": 1,
                "user": {"$arrayElemAt": ["$user", 0]}
            }
        };

        let pipeline = vec![
            filter,
            sort,
            join_user,
            select_first_user_and_remove_user_id,
        ];

        let db = self.db_connection.get_pool().await;
        let result = db
            .collection::<InparkModel>("inpark")
            .aggregate(pipeline, None)
            .await
            .map_err(|error| {
                log::error!("{error}");
                error
            })?;

        let results: Vec<Document> = result.try_collect().await.map_err(|error| {
            log::error!("{error}");
            error
        })?;

        let entities = results
            .into_iter()
            .map(bson::from_document)
            .collect::<Result<Vec<DetailedInpark>, bson::de::Error>>()
            .map_err(|error| {
                log::error!("{error}");
                error
            })?;

        Ok(entities)
    }
}
