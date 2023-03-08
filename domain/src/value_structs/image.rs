use std::fs::File;

#[non_exhaustive]
pub struct Image {
    pub data: File,
    pub content_type: String,
    pub file_name: String,
}

impl Image {
    pub fn new(data: File, content_type: &str, file_name: &str) -> Self {
        Self {
            data,
            content_type: content_type.to_string(),
            file_name: file_name.to_string(),
        }
    }
}
