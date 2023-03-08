use async_trait::async_trait;
use domain::entities::park_entity::ParkEntity;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ParksRepositoryAbstract {
    async fn get_all_parks(&self) -> Result<Vec<ParkEntity>, Box<dyn std::error::Error>>;
}
