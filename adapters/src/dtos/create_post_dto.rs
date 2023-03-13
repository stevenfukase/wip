use actix_multipart::form::{
    tempfile::TempFile,
    // text::Text,
    MultipartForm,
};

#[derive(MultipartForm)]
pub struct CreatePostDto {
    #[multipart(rename = "image")]
    pub images: Vec<TempFile>,
    // pub text: Text<String>,
}
