use crate::storage::shared::storage::Storage;
use application::repositories::images_repository_abstract::{ImageType, ImagesRepositoryAbstract};
use async_trait::async_trait;
use cloud_storage::Object;
use domain::value_structs::{image::Image, image_url::ImageUrl};
use futures::future::try_join_all;
use std::{
    collections::HashMap,
    error::Error,
    io::{BufReader, Read},
};

pub struct ImagesRepository {
    pub storage: Storage,
}

// Rust: Returning a future that takes reference to a string
// https://stackoverflow.com/q/75703061/15617266
#[async_trait(?Send)]
impl ImagesRepositoryAbstract for ImagesRepository {
    async fn upload_images(
        &self,
        images: &[Image],
        image_type: &ImageType,
    ) -> Result<Vec<ImageUrl>, Box<dyn Error>> {
        let image_type_map =
            HashMap::from([(ImageType::Post, "post"), (ImageType::Profile, "profile")]);

        let futures = images.iter().map(|image: &Image| async {
            let image_as_u8_vec = BufReader::new(&image.data)
                .bytes()
                .collect::<Result<Vec<u8>, _>>()?;

            let path = *image_type_map.get(image_type).ok_or("Invalid image type")?;
            let file_path_name = format!("{}/{}", path, &image.file_name);

            Ok::<Result<Object, cloud_storage::Error>, Box<dyn Error>>(
                self.storage
                    .upload(image_as_u8_vec, &file_path_name, &image.content_type)
                    .await,
            )
        });

        let results = try_join_all(futures)
            .await?
            .into_iter()
            .collect::<Result<Vec<Object>, cloud_storage::Error>>()?;

        let image_urls = results
            .iter()
            .map(|object| ImageUrl::from(&*object.self_link))
            .collect::<Vec<ImageUrl>>();

        Ok(image_urls)
    }
}
