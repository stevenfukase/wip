use super::parks_model::ParksModel;
use application::mappers::db_mapper::DbMapper;
use domain::{entities::park_entity::ParkEntity, value_structs::id::ParkId};
use std::str::FromStr;

pub struct ParksDbMapper {}

impl DbMapper<ParkEntity, ParksModel> for ParksDbMapper {
    fn to_db(entity: &ParkEntity) -> ParksModel {
        ParksModel {
            id: bson::oid::ObjectId::from_str(&entity.id.to_string()).unwrap(),
            name: entity.name.to_string(),
            name_ja: entity.name_ja.to_string(),
        }
    }

    fn to_entity(model: &ParksModel) -> ParkEntity {
        ParkEntity::new(&ParkId::from(model.id), &model.name, &model.name_ja)
    }
}
