use super::{
    app_state::AppState, auth_middleware::AuthMiddlewareFactory, grants_extractor, routes::routes,
};
use crate::{
    auth::{auth_repository::AuthRepository, config::AuthConfig},
    database::{
        inpark::inpark_repository::InparkRepository, parks::parks_repository::ParksRepository,
        posts::posts_repository::PostsRepository, shared::db_connection::DbConnection,
        users::users_repository::UsersRepository,
    },
    storage::{images::images_repository::ImagesRepository, shared::storage::Storage},
};
use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web, App, HttpServer,
};
use actix_web_grants::GrantsMiddleware;
use std::{net::TcpListener, sync::Arc};

pub fn run<'a>(
    listener: TcpListener,
    frontend_url: &str,
    db_url: &str,
    db_name: &str,
    bucket_name: &'a str,
    auth_config: AuthConfig,
) -> Result<actix_web::dev::Server, std::io::Error> {
    env_logger::init();

    let db_instance = DbConnection {
        db_url: db_url.to_string(),
        db_name: db_name.to_string(),
    };

    let db_connection = Arc::new(db_instance);
    let auth_config_arc = Arc::new(auth_config);

    let origin = frontend_url.to_string();

    let bucket_name_string = bucket_name.to_string();

    let factory = move || {
        let logger = Logger::default();
        let storage = Storage::new(&bucket_name_string);

        let data = web::Data::new(AppState {
            auth_repository: AuthRepository {
                auth_config: Arc::clone(&auth_config_arc),
            },
            images_repository: ImagesRepository { storage },
            inpark_repository: InparkRepository {
                db_connection: Arc::clone(&db_connection),
            },
            parks_repository: ParksRepository {
                db_connection: Arc::clone(&db_connection),
            },
            posts_repository: PostsRepository {
                db_connection: Arc::clone(&db_connection),
            },
            users_repository: UsersRepository {
                db_connection: Arc::clone(&db_connection),
            },
        });

        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .wrap(
                Cors::default()
                    .allowed_origin(&origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .supports_credentials()
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(logger)
            // actix-web-grants
            .wrap(GrantsMiddleware::with_extractor(grants_extractor::extract))
            // attach user info for access from controllers
            .wrap(AuthMiddlewareFactory)
            .configure(routes)
            .app_data(data)
    };

    let server = HttpServer::new(factory).listen(listener)?.run();

    Ok(server)
}
