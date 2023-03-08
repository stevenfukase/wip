use super::users_model::UserModel;
use application::mappers::db_mapper::DbMapper;
use bson::oid::ObjectId;
use domain::{
    entities::user_entity::UserEntityWithPassword,
    value_structs::{hashed_password::HashedPassword, id::UserId},
};
use std::str::FromStr;

pub struct UserDbMapper {}

impl DbMapper<UserEntityWithPassword, UserModel> for UserDbMapper {
    fn to_db(entity: &UserEntityWithPassword) -> UserModel {
        UserModel {
            id: None,
            username: entity.username.to_string(),
            nickname: entity.nickname.to_string(),
            password: entity.password.to_string(),
            email: entity.email.to_string(),
            friends: entity
                .friends
                .iter()
                .map(|user_id| ObjectId::from_str(&user_id.to_string()).unwrap())
                .collect(),
        }
    }

    fn to_entity(model: &UserModel) -> UserEntityWithPassword {
        let friends = model
            .friends
            .iter()
            .map(|object_id| UserId::from(*object_id))
            .collect::<Vec<UserId>>();

        UserEntityWithPassword::new(
            &UserId::from(model.id.unwrap_or_default()),
            &model.username,
            &model.nickname,
            &HashedPassword::from(&*model.password),
            &model.email,
            &friends,
        )
    }
}
