use async_trait::async_trait;
use domain::{
    entities::post_entity::PostEntity,
    value_structs::{hashtag::Hashtag, id::UserId, image_url::ImageUrl},
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::{collections::HashSet, error::Error};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait PostsRepositoryAbstract {
    async fn insert_post(
        &self,
        user_id: &UserId,
        images_urls: &[ImageUrl; 4],
        text: &str,
        hashtags: &HashSet<Hashtag>,
    ) -> Result<PostEntity, Box<dyn Error>>;
}
