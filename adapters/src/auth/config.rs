use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
}
