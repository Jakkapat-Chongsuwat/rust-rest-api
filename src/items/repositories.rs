use bson::doc;
use crate::config::database::Connection;
use super::models::InserItemReq;
use bson::oid::ObjectId;
use std::sync::Arc;

#[derive(Clone)]  
pub struct ItemRepository {
    db_connection: Arc<dyn Connection + Send + Sync>,
}

impl ItemRepository {
    pub fn new(db_connection: Arc<dyn Connection + Send + Sync>) -> Self {
        Self { db_connection }
    }

    pub async fn insert_one_item(
        &self, 
        req: InserItemReq,
    ) -> Result<ObjectId, String> {
        let col = self.db_connection.collection("items");

        let result = col
            .insert_one(
                doc! {
                    "name": req.name,
                    "description": req.description,
                    "damage": req.damage,
                    "level_required": req.level_required,
                    "price": req.price,
                },
            )
            .await
            .map_err(|e| format!("Error inserting item: {}", e))?;

        let inserted_id_bson = result.inserted_id;

        inserted_id_bson
            .as_object_id()
            .ok_or_else(|| String::from("Error getting inserted id"))
    }
}
