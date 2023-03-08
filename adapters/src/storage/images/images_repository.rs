use crate::storage::shared::storage::Storage;
use application::repositories::images_repository_abstract::{ImageType, ImagesRepositoryAbstract};
use async_trait::async_trait;
use cloud_storage::Object;
use domain::value_structs::{image::Image, image_url::ImageUrl};
use futures::{future::try_join_all, Future};
use std::{error::Error, io::BufReader, pin::Pin};

pub struct ImagesRepository {
    pub storage: Storage,
}

#[async_trait(?Send)]
impl ImagesRepositoryAbstract for ImagesRepository {
    async fn upload_images(
        &self,
        images: &[Image],
        image_type: ImageType,
    ) -> Result<[ImageUrl; 4], Box<dyn Error>> {
        let futures = images
            .into_iter()
            .map(
                |image: &Image| -> Result<
                    Pin<Box<dyn Future<Output = Result<Object, cloud_storage::Error>>>>,
                    Box<dyn Error>,
                > {
                    let image_clone = image.data.try_clone().or(Err("Cannot clone image"))?;
                    let image_as_u8_vec = BufReader::new(image_clone).buffer().to_vec();

                    Ok(Box::pin(self.storage.upload(
                        image_as_u8_vec,
                        &image.file_name,
                        &image.content_type,
                    )))
                },
            )
            .collect::<Result<
                Vec<Pin<Box<dyn Future<Output = Result<Object, cloud_storage::Error>>>>>,
                Box<dyn Error>,
            >>()?;

        let results = try_join_all(futures);
        todo!()
        // Err("test".into())
    }
}
