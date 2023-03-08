use actix_web::{dev, FromRequest, HttpMessage, HttpRequest};
use domain::entities::claims_entity::Claims;
use futures::future::ready;
use futures_util::future;

#[non_exhaustive]
#[derive(Debug)]
pub struct User(pub Claims);

impl User {
    pub fn get_claims(&self) -> &Claims {
        &self.0
    }
}

impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let extensions = req.extensions();
        let claims = extensions.get::<Claims>();

        let result = match claims {
            Some(claims) => Ok(User(claims.to_owned())),
            None => Err(actix_web::error::ErrorBadRequest("")),
        };

        ready(result)
    }
}
