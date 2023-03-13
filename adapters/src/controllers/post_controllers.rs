use crate::{
    dtos::create_post_dto::CreatePostDto,
    infrastructure::{app_state::AppState, auth_extractor::User},
    presenters::{error_presenter, post_presenter},
};
use actix_multipart::form::MultipartForm;
use actix_web::{post, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use application::{
    mappers::presenter::Presenter,
    use_cases::{abstract_use_case::AbstractUseCase, create_post_use_case::CreatePostUseCase},
};
use domain::{
    common::error::ApiError,
    enums::role::Role::{self, USER},
    value_structs::image::Image,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(create_post);
}

#[post("/")]
#[has_any_role("USER", type = "Role")]
async fn create_post(
    data: web::Data<AppState>,
    MultipartForm(form): MultipartForm<CreatePostDto>,
    user: User,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let posts_repository = &data.posts_repository;
    let images_repository = &data.images_repository;

    let user_id = &user.get_claims().uid;

    let images = form
        .images
        .into_iter()
        .map(|temp_file| {
            let content_type =
                temp_file
                    .content_type
                    .ok_or(ApiError::new(400, "Content type not found", None))?;

            let essence_str = content_type.essence_str();

            let file_name =
                temp_file
                    .file_name
                    .ok_or(ApiError::new(400, "File name not found", None))?;

            let file = temp_file.file.into_file();

            Ok(Image::new(file, essence_str, &file_name))
        })
        .collect::<Result<Vec<Image>, _>>()
        .map_err(error_presenter::ErrorResponse::map_io_error)?;

    let create_post_use_case = CreatePostUseCase::new(
        posts_repository,
        images_repository,
        user_id,
        &images,
        // &form.text,
    );

    let post = create_post_use_case.execute().await;

    post.map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|post| HttpResponse::Ok().json(post_presenter::PostPresenter::to_api(&post)))
}
