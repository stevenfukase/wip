use adapters::{auth::config::AuthConfig, infrastructure::run};
use std::{fs, io, net};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let address = dotenv::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:80".to_string());
    let db_name = dotenv::var("DB_NAME").expect("DB_NAME must be set");
    let db_url = dotenv::var("DB_URL").expect("DB_URL must be set");
    let frontend_url = dotenv::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let bucket_name = dotenv::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

    let listener = net::TcpListener::bind(address).expect("Failed to bind port");

    let file = fs::File::open("firebase.json").expect("firebase.json not found");
    let reader = io::BufReader::new(file);
    let firebase_config: AuthConfig =
        serde_json::from_reader(reader).expect("firebase.json is incorrect");

    run::run(
        listener,
        &frontend_url,
        &db_url,
        &db_name,
        &bucket_name,
        firebase_config,
    )?
    .await
}
