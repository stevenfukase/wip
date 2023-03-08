use application::mappers::presenter::Presenter;
use domain::value_structs::date_time;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize)]
pub struct DatePresenter(String);

impl Presenter<date_time::DateTime> for DatePresenter {
    fn to_api(date: &date_time::DateTime) -> Self {
        Self(date.to_bson().try_to_rfc3339_string().unwrap_or_default())
    }
}
