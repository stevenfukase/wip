use application::mappers::presenter::Presenter;
use domain::entities::user_entity::UserEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPresenter {
    pub username: String,
    pub nickname: String,
}

impl Presenter<UserEntity> for UserPresenter {
    fn to_api(entity: &UserEntity) -> UserPresenter {
        UserPresenter {
            username: entity.username.to_string(),
            nickname: entity.nickname.to_string(),
        }
    }
}
