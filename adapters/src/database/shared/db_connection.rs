use mongodb::{Client, Database};

// Pool handled by driver
type DbPool = Database;

#[derive(Clone, Debug)]
pub struct DbConnection {
    pub db_url: String,
    pub db_name: String,
}

impl DbConnection {
    pub async fn get_pool(&self) -> DbPool {
        let client = Client::with_uri_str(&self.db_url).await.unwrap();
        client.database(&self.db_name)
    }
}
