use super::{users_db_mapper::UserDbMapper, users_model::UserModel};
use crate::database::shared::db_connection::DbConnection;
use application::{
    mappers::db_mapper::DbMapper, repositories::users_repository_abstract::UsersRepositoryAbstract,
};
use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use domain::{
    entities::user_entity::{UserEntity, UserEntityWithPassword},
    value_structs::id::UserId,
};
use std::{str::FromStr, sync::Arc};

pub struct UsersRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl UsersRepositoryAbstract for UsersRepository {
    async fn get_user_by_id(
        &self,
        user_id: &UserId,
    ) -> Result<UserEntityWithPassword, Box<dyn std::error::Error>> {
        let db = self.db_connection.get_pool().await;
        let collection = db.collection::<UserModel>("users");

        let user_object_id = ObjectId::from_str(&user_id.to_string())?;
        let result = collection
            .find_one(doc! { "_id": user_object_id }, None)
            .await?;

        match result {
            Some(model) => Ok(UserDbMapper::to_entity(&model)),
            None => Err("User not found")?,
        }
    }

    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<UserEntityWithPassword, Box<dyn std::error::Error>> {
        let db = self.db_connection.get_pool().await;
        let collection = db.collection::<UserModel>("users");
        let result = collection
            .find_one(doc! { "username": username }, None)
            .await?;

        match result {
            Some(model) => Ok(UserDbMapper::to_entity(&model)),
            None => {
                log::info!("{:?}", result);
                Err("User not found")?
            }
        }
    }

    async fn insert_new_user(
        &self,
        username: &str,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<UserEntity, Box<dyn std::error::Error>> {
        let user_to_insert = UserModel::new(&None, username, nickname, password, email, &[]);

        let db = self.db_connection.get_pool().await;
        let collection = db.collection::<UserModel>("users");
        let result = collection.insert_one(&user_to_insert, None).await?;

        let user_id = result.inserted_id.as_object_id();

        if user_id.is_none() {
            return Err("User not inserted".into());
        }

        let new_user = UserEntity::new(
            &user_id.unwrap().to_string(),
            username,
            nickname,
            email,
            &[],
        );

        Ok(new_user)
    }

    async fn get_user_with_email_and_username(
        &self,
        username: &str,
        email: &str,
    ) -> Result<Option<UserEntity>, Box<dyn std::error::Error>> {
        let db = self.db_connection.get_pool().await;
        let collection = db.collection::<UserModel>("users");
        let filter = doc! {"$or": [{"username": username},{ "email": email}]};
        let result = collection.find_one(filter, None).await?;

        match result {
            Some(model) => Ok(Some(UserDbMapper::to_entity(&model).to_user_entity())),
            None => Ok(None),
        }
    }

    // async fn get_users_by_id(
    //     &self,
    //     user_ids: &[UserId],
    // ) -> Result<Vec<UserEntity>, Box<dyn std::error::Error>> {
    //     let user_id_strs = user_ids
    //         .iter()
    //         .map(|user_id| user_id.to_string())
    //         .collect::<Vec<String>>();

    //     let db = self.db_connection.get_pool().await;
    //     let collection = db.collection::<UserModel>("users");
    //     let filter = doc! {"_id": { "$in": user_id_strs } };
    //     let result = collection.find(filter, None).await?;

    //     let results: Vec<UserModel> = result.try_collect().await?;

    //     let entities = results
    //         .iter()
    //         .map(|model| UserDbMapper::to_entity(model).to_user_entity())
    //         .collect();

    //     Ok(entities)
    // }
}
