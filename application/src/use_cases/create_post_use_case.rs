use super::abstract_use_case::AbstractUseCase;
use crate::{
    repositories::{
        images_repository_abstract::{ImageType, ImagesRepositoryAbstract},
        posts_repository_abstract::PostsRepositoryAbstract,
    },
    utils::hashtags_extactor,
};
use async_trait::async_trait;
use domain::{
    common::error::ApiError,
    entities::post_entity::PostEntity,
    enums::create_post_error::CreatePostError,
    value_structs::{id::UserId, image::Image},
};

pub struct CreatePostUseCase<'a> {
    pub posts_repository: &'a dyn PostsRepositoryAbstract,
    pub images_repository: &'a dyn ImagesRepositoryAbstract,
    pub user_id: &'a UserId,
    pub images: &'a Vec<Image>,
    pub text: &'a str,
}

impl<'a> CreatePostUseCase<'a> {
    pub fn new(
        posts_repository: &'a dyn PostsRepositoryAbstract,
        images_repository: &'a dyn ImagesRepositoryAbstract,
        user_id: &'a UserId,
        images: &'a Vec<Image>,
        // text: &'a str,
    ) -> Self {
        Self {
            posts_repository,
            images_repository,
            user_id,
            images,
            text: "",
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<PostEntity> for CreatePostUseCase<'a> {
    async fn execute(&self) -> Result<PostEntity, ApiError> {
        if self.images.len() > 4 {
            return Err(CreatePostError::ImageMoreThanFour.into());
        }

        let image_urls = self
            .images_repository
            .upload_images(self.images, ImageType::Post)
            .await?;

        let hashtags = hashtags_extactor::extract(self.text);

        let result = self
            .posts_repository
            .insert_post(self.user_id, &image_urls, self.text, &hashtags)
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[actix_rt::test]
    async fn can_create_post() {
        //
    }
}
