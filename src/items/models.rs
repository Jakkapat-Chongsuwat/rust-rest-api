use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InserItemReq {
    pub name: String,
    pub description: String,
    pub damage: i32,
    pub level_required: i32,
    pub price: i32,
}