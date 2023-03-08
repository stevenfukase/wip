use crate::controllers::{
    inpark_controllers, park_controllers, post_controllers, user_controllers,
};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(web::scope("/inparks").configure(inpark_controllers::routes))
            .service(web::scope("/parks").configure(park_controllers::routes))
            .service(web::scope("/posts").configure(post_controllers::routes))
            .service(web::scope("/users").configure(user_controllers::routes)),
    );
}
