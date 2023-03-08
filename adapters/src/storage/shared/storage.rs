use cloud_storage::{Client, Error, Object};

#[non_exhaustive]
pub struct Storage {
    pub client: Client,
    bucket_name: String,
}

impl Storage {
    pub fn new(bucket_name: &str) -> Self {
        Storage {
            client: Client::default(),
            bucket_name: bucket_name.to_string(),
        }
    }

    pub async fn upload(
        &self,
        file: Vec<u8>,
        filename: &str,
        mime_type: &str,
    ) -> Result<Object, Error> {
        self.client
            .object()
            .create(&self.bucket_name, file, filename, mime_type)
            .await
    }

    pub async fn download(&self, bucket: &str, file_name: &str) -> Result<Object, Error> {
        self.client.object().read(bucket, file_name).await
    }

    // pub async fn delete(&self, file_name: &str) -> Result<(), Error> {
    //     self.client
    //         .object()
    //         .delete(&self.bucket_name, file_name)
    //         .await
    // }
}
