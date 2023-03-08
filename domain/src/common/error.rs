use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub error: Option<Box<dyn Error>>,
}

impl ApiError {
    pub fn new(code: u16, message: &str, error: Option<Box<dyn Error>>) -> Self {
        Self {
            code,
            message: message.to_string(),
            error,
        }
    }

    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }

    pub fn get_error_code(&self) -> u16 {
        self.code
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!")
    }
}

impl From<Box<dyn Error>> for ApiError {
    fn from(value: Box<dyn Error>) -> Self {
        Self {
            error: Some(value),
            code: 400,
            message: "API Error".to_string(),
        }
    }
}
