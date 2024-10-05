use bson::Document;
use mongodb::{options::ClientOptions, Client, Collection, Database as MongoDatabase};
use async_trait::async_trait;
use std::sync::Arc;
use crate::config::database::{DatabaseConnection, Connection};

pub struct MongoDbConnection {
    uri: String,
}

impl MongoDbConnection {
    pub fn new(uri: String) -> Self {
        MongoDbConnection { uri }
    }
}

#[async_trait]
impl DatabaseConnection for MongoDbConnection {
    async fn connect(&self) -> Arc<dyn Connection + Send + Sync> {
        let mut client_options = ClientOptions::parse(&self.uri).await.unwrap();
        client_options.app_name = Some("My App".to_string());

        let client = Client::with_options(client_options).unwrap();
        let db = client.database("rust-rest-db");

        Arc::new(MongoConnection { db })
    }
}

pub struct MongoConnection {
    db: MongoDatabase,
}

#[async_trait]
impl Connection for MongoConnection {
    fn execute_query(&self, query: &str) -> Result<(), String> {
        println!("Executing MongoDB query: {}", query);
        Ok(())
    }
    fn collection(&self, name: &str) -> Collection<Document> {
        self.db.collection::<Document>(name)
    }
}
