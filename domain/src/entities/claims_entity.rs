use crate::{enums::role::Role, value_structs::id::UserId};
use chrono::Utc;
use serde::{Deserialize, Serialize};

const JWT_EXPIRATION_SECONDS: i64 = 3600;
const AUDIENCE: &str =
    "https://identitytoolkit.googleapis.com/google.identity.identitytoolkit.v1.IdentityToolkit";

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    // audience
    pub aud: String,
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    pub uid: UserId,
    pub permissions: Vec<Role>,
}

impl Claims {
    pub fn new(client_email: &str, uid: &UserId, permissions: &[Role]) -> Self {
        let now = Utc::now().timestamp();
        let exp = now + JWT_EXPIRATION_SECONDS;

        Self {
            iss: client_email.to_owned(),
            sub: client_email.to_owned(),
            aud: AUDIENCE.to_owned(),
            iat: now,
            exp,
            uid: uid.to_owned(),
            permissions: permissions.to_owned(),
        }
    }

    pub fn from_other_format(
        iss: String,
        sub: String,
        aud: String,
        iat: i64,
        exp: i64,
        uid: UserId,
        permissions: Vec<Role>,
    ) -> Self {
        Self {
            iss,
            sub,
            aud,
            iat,
            exp,
            uid,
            permissions,
        }
    }
}
