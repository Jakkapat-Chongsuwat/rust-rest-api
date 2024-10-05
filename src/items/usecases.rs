use bson::oid::ObjectId;
use super::repositories::ItemRepository;
use super::models::InserItemReq;
use std::sync::Arc;

#[derive(Clone)]  
pub struct ItemUseCase {
    repository: Arc<ItemRepository>,
}

impl ItemUseCase {
    pub fn new(repository: Arc<ItemRepository>) -> Self {
        Self { repository }
    }

    pub async fn insert_one_item(
        &self, 
        req: InserItemReq,
    ) -> Result<ObjectId, String> {
        self.repository.insert_one_item(req).await
    }
}
