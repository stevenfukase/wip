use crate::database::shared::db_connection::DbConnection;
use application::repositories::posts_repository_abstract::PostsRepositoryAbstract;
use async_trait::async_trait;
use bson::doc;
use domain::{
    entities::post_entity::PostEntity,
    value_structs::{
        hashtag::Hashtag,
        id::{PostId, UserId},
        image_url::ImageUrl,
    },
};
use std::{collections::HashSet, sync::Arc};

pub struct PostsRepository {
    pub db_connection: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl PostsRepositoryAbstract for PostsRepository {
    async fn insert_post(
        &self,
        user_id: &UserId,
        image_urls: &[ImageUrl],
        text: &str,
        hashtags: &HashSet<Hashtag>,
    ) -> Result<PostEntity, Box<dyn std::error::Error>> {
        let db = self.db_connection.get_pool().await;
        let collection = db.collection("posts");

        let doc = doc! {
            "user_id": user_id.to_string(),
            "image_urls": image_urls.to_vec(),
            "text": text,
            "likers": [],
            "hashtags": hashtags.iter().map(Hashtag::to_string).collect::<Vec<String>>(),
        };

        let result = collection.insert_one(doc, None).await?;

        let post_id = result.inserted_id.as_object_id();

        if post_id.is_none() {
            return Err("Post not inserted".into());
        }

        let new_post = PostEntity::new(
            &PostId::from(post_id.unwrap()),
            user_id,
            image_urls,
            text,
            &[],
            hashtags,
        )?;

        Ok(new_post)
    }
}
