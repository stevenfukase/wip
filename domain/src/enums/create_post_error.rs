use crate::common::error::ApiError;

#[derive(Debug)]
pub enum CreatePostError {
    ImageMoreThanFour,
}

impl std::fmt::Display for CreatePostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreatePostError::ImageMoreThanFour => write!(f, "More than 4 images were sent"),
        }
    }
}

impl std::error::Error for CreatePostError {}

impl From<CreatePostError> for ApiError {
    fn from(value: CreatePostError) -> Self {
        match value {
            CreatePostError::ImageMoreThanFour => Self {
                error: Some(Box::from(CreatePostError::ImageMoreThanFour)),
                code: 400,
                message: CreatePostError::ImageMoreThanFour.to_string(),
            },
        }
    }
}
