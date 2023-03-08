use application::mappers::presenter::Presenter;
use domain::{entities::park_entity::ParkEntity, value_structs::id::ParkId};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
pub struct ParkPresenter {
    id: ParkId,
    name: String,
    name_ja: String,
}

impl Presenter<ParkEntity> for ParkPresenter {
    fn to_api(park_entity: &ParkEntity) -> Self {
        Self {
            id: park_entity.id.to_owned(),
            name: park_entity.name.to_owned(),
            name_ja: park_entity.name_ja.to_owned(),
        }
    }
}
