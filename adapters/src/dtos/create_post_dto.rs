use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

#[derive(MultipartForm)]
pub struct CreatePostDto {
    pub images: Vec<TempFile>,
    // pub text: Text<String>,
}
