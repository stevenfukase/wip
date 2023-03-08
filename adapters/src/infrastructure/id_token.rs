use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdToken {
    // expiration
    pub exp: i64,
    // issued at
    pub iat: i64,
    // audience
    pub aud: String,
    // issuer
    pub iss: String,
    // subject
    pub sub: String,
    pub auth_time: i64,
}
