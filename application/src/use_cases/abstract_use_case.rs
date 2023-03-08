use async_trait::async_trait;
use domain::common::error::ApiError;

#[async_trait(?Send)]
pub trait AbstractUseCase<T> {
    async fn execute(&self) -> Result<T, ApiError>;
}
