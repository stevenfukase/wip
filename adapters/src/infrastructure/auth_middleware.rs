use super::app_state::AppState;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, HttpMessage,
};
use application::repositories::auth_repository_abstract::AuthRepositoryAbstract;
use domain::{entities::claims_entity::Claims, value_structs::token::Token};
use futures_util::{future::LocalBoxFuture, FutureExt};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

// `S` - type of the next service
// `B` - type of response's body

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        async move {
            let data = req.app_data::<web::Data<AppState>>().unwrap();
            let header_value = req.headers().get("authorization");
            if let Some(bearer_token_header_value) = header_value {
                let bearer_token = bearer_token_header_value.to_str().unwrap();
                let result = data
                    .auth_repository
                    .decode_token(&Token::new(bearer_token))
                    .await;

                match result {
                    Ok(claims) => {
                        req.extensions_mut().insert::<Claims>(claims);
                    }
                    Err(error) => {
                        log::error!("{error}");
                    }
                }
            }

            let res = service.call(req).await?;
            Ok(res)
        }
        .boxed_local()
    }
}

pub struct AuthMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}
