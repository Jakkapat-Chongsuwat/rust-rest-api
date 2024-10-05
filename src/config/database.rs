use async_trait::async_trait;
use bson::Document;
use mongodb::Collection;
use std::sync::Arc;

#[async_trait]
pub trait DatabaseConnection {
    async fn connect(&self) -> Arc<dyn Connection + Send + Sync>;
}

pub trait Connection {
    fn execute_query(&self, query: &str) -> Result<(), String>;
    fn collection(&self, name: &str) -> Collection<Document>;
}
