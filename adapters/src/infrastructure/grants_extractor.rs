use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use domain::{entities::claims_entity::Claims, enums::role::Role};

pub async fn extract(req: &ServiceRequest) -> Result<Vec<Role>, Error> {
    if let Some(claims) = req.extensions().get::<Claims>() {
        return Ok(claims.permissions.clone());
    }

    Ok(vec![])
}
