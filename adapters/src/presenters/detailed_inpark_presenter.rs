use super::date_presenter::DatePresenter;
use application::mappers::presenter::Presenter;
use domain::{
    aggregates::detailed_inpark::DetailedInpark,
    entities::user_entity::UserEntity,
    value_structs::id::{InparkId, ParkId},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DetailedInparkPresenter {
    pub id: InparkId,
    pub user: UserEntity,
    pub park_id: ParkId,
    pub date_time: DatePresenter,
}

impl Presenter<DetailedInpark> for DetailedInparkPresenter {
    fn to_api(entity: &DetailedInpark) -> DetailedInparkPresenter {
        let date_time = DatePresenter::to_api(&entity.date_time);

        DetailedInparkPresenter {
            id: entity.id.clone(),
            user: entity.user.clone(),
            park_id: entity.park_id.clone(),
            date_time,
        }
    }
}
