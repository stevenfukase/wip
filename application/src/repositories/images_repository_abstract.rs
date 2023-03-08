use async_trait::async_trait;
use domain::value_structs::{image::Image, image_url::ImageUrl};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

pub enum ImageType {
    Profile,
    Post,
}

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ImagesRepositoryAbstract {
    async fn upload_images(
        &self,
        images: &[Image],
        image_type: ImageType,
    ) -> Result<[ImageUrl; 4], Box<dyn Error>>;
}
