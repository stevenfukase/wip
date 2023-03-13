use async_trait::async_trait;
use domain::value_structs::{image::Image, image_url::ImageUrl};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[derive(PartialEq, Eq, Hash)]
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
        image_type: &ImageType,
    ) -> Result<Vec<ImageUrl>, Box<dyn Error>>;
}
