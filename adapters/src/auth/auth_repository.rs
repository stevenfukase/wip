use super::config::AuthConfig;
use crate::{constants, infrastructure::id_token::IdToken};
use application::repositories::auth_repository_abstract::AuthRepositoryAbstract;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use async_trait::async_trait;
use domain::{
    entities::claims_entity::Claims,
    enums::role::Role,
    value_structs::{hashed_password::HashedPassword, id::UserId, token::Token},
};
use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use std::{collections::HashMap, error::Error, sync::Arc};

pub struct AuthRepository {
    pub auth_config: Arc<AuthConfig>,
}

#[async_trait(?Send)]
impl AuthRepositoryAbstract for AuthRepository {
    fn create_token(
        &self,
        user_id: &UserId,
        roles: &[Role],
    ) -> Result<(Token, i64), Box<dyn Error>> {
        let auth_config = &self.auth_config;
        let client_email = &auth_config.client_email;
        let private_key = &auth_config.private_key;
        let private_key_id = &auth_config.private_key_id;

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(private_key_id.to_string());

        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())?;

        let claims = Claims::new(client_email, user_id, roles);
        let token = jsonwebtoken::encode(&header, &claims, &encoding_key)?;

        Ok((Token::new(&token), claims.exp))
    }

    async fn decode_token(&self, token: &Token) -> Result<Claims, Box<dyn Error>> {
        let auth_config = &self.auth_config;
        let project_id = &auth_config.project_id;
        let public_key_url = constants::FIREBASE_PUBLIC_KEY_URL;

        let private_kid = jsonwebtoken::decode_header(&token.to_string())?
            .kid
            .ok_or("Cannot decode header")?;

        let key_map = reqwest::get(public_key_url)
            .await?
            .json::<HashMap<String, String>>()
            .await?;

        let kid = key_map.get(&private_kid).ok_or("Key does not exist")?;
        let certificate = openssl::x509::X509::from_pem(kid.as_bytes())?;
        let pem_bytes = certificate.public_key()?.rsa()?.public_key_to_pem()?;

        let mut validation = Validation::new(Algorithm::RS256);

        validation.set_issuer(&[format!("https://securetoken.google.com/{project_id}")]);
        validation.set_audience(&[project_id]);

        let decoding_key = DecodingKey::from_rsa_pem(&pem_bytes)?;

        let token_data_claims =
            jsonwebtoken::decode::<IdToken>(&token.to_string(), &decoding_key, &validation)?.claims;

        // Todo: get roles
        let claims = Claims::from_other_format(
            token_data_claims.iss,
            token_data_claims.sub.clone(),
            token_data_claims.aud,
            token_data_claims.iat,
            token_data_claims.exp,
            UserId::from(&*token_data_claims.sub),
            [Role::USER].to_vec(),
        );

        Ok(claims)
    }

    fn hash_password(&self, password: &str) -> Result<HashedPassword, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed_password = argon2.hash_password(password.as_bytes(), &salt);

        match hashed_password {
            Ok(hashed_string) => Ok(HashedPassword::from(&*hashed_string.to_string())),
            Err(e) => Err(e.to_string().into()),
        }
    }

    fn hashes_match(
        &self,
        hashed_password: &HashedPassword,
        password: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let hashed_password_string = hashed_password.to_string();
        let parsed_hash =
            PasswordHash::new(&hashed_password_string).map_err(|error| error.to_string())?;

        let is_match = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_match)
    }
}
