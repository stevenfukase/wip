use crate::{
    infrastructure::app_state::AppState,
    presenters::{error_presenter, park_presenter::ParkPresenter},
};
use actix_web::{get, web, HttpResponse};
use application::{
    mappers::presenter::Presenter,
    use_cases::{abstract_use_case::AbstractUseCase, get_all_parks_use_case::GetAllParksUseCase},
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(get_all_parks);
}

#[get("/")]
async fn get_all_parks(
    data: web::Data<AppState>,
) -> Result<HttpResponse, error_presenter::ErrorResponse> {
    let get_all_parks_usecase = GetAllParksUseCase::new(&data.parks_repository);
    let parks = get_all_parks_usecase.execute().await;

    parks
        .map_err(error_presenter::ErrorResponse::map_io_error)
        .map(|parks| {
            let park_presenters = parks
                .iter()
                .map(ParkPresenter::to_api)
                .collect::<Vec<ParkPresenter>>();

            HttpResponse::Ok().json(park_presenters)
        })
}
