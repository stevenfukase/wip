use crate::{
    dtos::create_inpark_dto::CreateInparkDto,
    infrastructure::{app_state::AppState, auth_extractor::User},
    presenters::{
        detailed_inpark_presenter::DetailedInparkPresenter, error_presenter, inpark_id_presenter,
    },
};
use actix_web::{get, post, web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use application::{
    mappers::presenter::Presenter,
    use_cases::{
        abstract_use_case::AbstractUseCase, create_inpark_use_case::CreateSingleInparkUseCase,
        get_timeline_use_case::GetTimelineUseCase,
    },
};
use domain::{
    enums::role::Role::{self, USER},
    value_structs::date_time::DateTime,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(post_single_inpark).service(get_timeline);
}

#[post("/")]
#[has_any_role("USER", type = "Role")]
async fn post_single_inpark(
    data: web::Data<AppState>,
    form: web::Json<CreateInparkDto>,
    user: User,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let user_id = &user.get_claims().uid;
    let date_time = DateTime::from(form.date_time);
    let post_single_inpark_use_case =
        CreateSingleInparkUseCase::new(&data.inpark_repository, &form.park_id, user_id, &date_time);
    let inpark = post_single_inpark_use_case.execute().await;

    inpark
        .map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|id| HttpResponse::Ok().json(inpark_id_presenter::InparkIdPresenter::to_api(&id)))
}

#[get("/timeline/")]
#[has_any_role("USER", type = "Role")]
async fn get_timeline(
    data: web::Data<AppState>,
    user: User,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let user_id = &user.get_claims().uid;

    let get_timeline_use_case =
        GetTimelineUseCase::new(&data.inpark_repository, &data.users_repository, user_id);
    let timeline = get_timeline_use_case.execute().await;

    timeline
        .map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|inparks| {
            let inpark_presenters = inparks
                .iter()
                .map(DetailedInparkPresenter::to_api)
                .collect::<Vec<DetailedInparkPresenter>>();

            HttpResponse::Ok().json(inpark_presenters)
        })
}
