use crate::{
    dtos::{create_user_dto::CreateUserDto, user_login_dto::UserLoginDto},
    infrastructure::app_state::AppState,
    presenters::{error_presenter, user_presenter::UserPresenter},
};
use actix_web::{get, post, web, HttpResponse};
use application::{
    mappers::presenter::Presenter,
    use_cases::{
        abstract_use_case::AbstractUseCase, create_user_session_use_case::CreateUserSessionUseCase,
        create_user_use_case::CreateUserUseCase,
        get_public_profile_use_case::GetPublicProfileUseCase,
    },
};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(get_public_profile)
        .service(create)
        .service(login);
}

#[get("/{username}/")]
async fn get_public_profile(
    data: web::Data<AppState>,
    username: web::Path<String>,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let extracted_username = username.into_inner();
    let get_public_profile_usecase =
        GetPublicProfileUseCase::new(&data.users_repository, &extracted_username);
    let user = get_public_profile_usecase.execute().await;

    user.map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|user| HttpResponse::Ok().json(UserPresenter::to_api(&user)))
}

#[post("/create/")]
async fn create(
    data: web::Data<AppState>,
    form: web::Json<CreateUserDto>,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let create_user_use_case = CreateUserUseCase::new(
        &data.users_repository,
        &data.auth_repository,
        &form.username,
        &form.nickname,
        &form.email,
        &form.password,
    );

    let user = create_user_use_case.execute().await;
    user.map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|user| HttpResponse::Ok().json(user))
}

#[post("/login/")]
async fn login(
    data: web::Data<AppState>,
    form: web::Json<UserLoginDto>,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let login_use_case = CreateUserSessionUseCase::new(
        &data.users_repository,
        &data.auth_repository,
        &form.username,
        &form.password,
    );

    let session = login_use_case.execute().await;

    session
        .map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|session| HttpResponse::Ok().json(session))
}
