use async_trait::async_trait;
use domain::{
    aggregates::detailed_inpark::DetailedInpark,
    value_structs::{
        date_time::DateTime,
        id::{InparkId, ParkId, UserId},
    },
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait InparkRepositoryAbstract {
    async fn create_inpark(
        &self,
        park_id: &ParkId,
        user_id: &UserId,
        date: &DateTime,
    ) -> Result<InparkId, Box<dyn Error>>;

    async fn get_detailed_inparks_by_multiple_users(
        &self,
        user_ids: &[UserId],
    ) -> Result<Vec<DetailedInpark>, Box<dyn Error>>;
}
