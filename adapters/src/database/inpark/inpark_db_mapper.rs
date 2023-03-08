use super::inpark_model::InparkModel;
use application::mappers::db_mapper::DbMapper;
use domain::{
    entities::inpark_entity::InparkEntity,
    value_structs::{date_time::DateTime, id::InparkId},
};
use std::str::FromStr;

pub struct InparkDbMapper {}

impl DbMapper<InparkEntity, InparkModel> for InparkDbMapper {
    fn to_db(entity: &InparkEntity) -> InparkModel {
        InparkModel {
            id: bson::oid::ObjectId::from_str(&entity.id.to_string()).unwrap(),
            park_id: entity.park_id.to_owned(),
            user_id: entity.user_id.to_owned(),
            date_time: entity.date_time.to_bson(),
        }
    }

    fn to_entity(model: &InparkModel) -> InparkEntity {
        InparkEntity::new(
            &InparkId::from(model.id),
            &model.user_id,
            &model.park_id,
            &DateTime::from(model.date_time),
        )
    }
}
