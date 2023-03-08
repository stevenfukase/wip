use crate::common::error::ApiError;

#[derive(Debug)]
pub enum CreateUserError {
    UsernameExists,
    EmailExists,
}

impl std::fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateUserError::UsernameExists => write!(f, "Username Exists"),
            CreateUserError::EmailExists => write!(f, "Email Exists"),
        }
    }
}

impl std::error::Error for CreateUserError {}

impl From<CreateUserError> for ApiError {
    fn from(value: CreateUserError) -> Self {
        match value {
            CreateUserError::EmailExists => Self {
                error: Some(Box::from(CreateUserError::EmailExists)),
                code: 400,
                message: CreateUserError::EmailExists.to_string(),
            },
            CreateUserError::UsernameExists => Self {
                error: Some(Box::from(CreateUserError::UsernameExists)),
                code: 400,
                message: CreateUserError::EmailExists.to_string(),
            },
        }
    }
}
