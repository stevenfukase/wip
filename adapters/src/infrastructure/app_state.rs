use crate::{
    auth::auth_repository::AuthRepository,
    database::{
        inpark::inpark_repository::InparkRepository, parks::parks_repository::ParksRepository,
        posts::posts_repository::PostsRepository, users::users_repository::UsersRepository,
    },
    storage::images::images_repository::ImagesRepository,
};

pub struct AppState {
    pub auth_repository: AuthRepository,
    pub inpark_repository: InparkRepository,
    pub images_repository: ImagesRepository,
    pub parks_repository: ParksRepository,
    pub posts_repository: PostsRepository,
    pub users_repository: UsersRepository,
}
