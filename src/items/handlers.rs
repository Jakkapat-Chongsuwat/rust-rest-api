use axum::{http::StatusCode, response::{IntoResponse, Json}, extract::State};
use axum_macros::debug_handler;
use super::usecases::ItemUseCase;
use super::models::InserItemReq;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemHandler {
    use_case: Arc<ItemUseCase>,
}

impl ItemHandler {
    pub fn new(use_case: Arc<ItemUseCase>) -> Self {
        Self { use_case }
    }

    // This is now a regular method, not a handler
    pub async fn process_insert_one_item(
        &self,
        req: InserItemReq,
    ) -> Result<impl IntoResponse, impl IntoResponse> {
        let result = self.use_case.insert_one_item(req).await;

        match result {
            Ok(inserted_id) => Ok((
                StatusCode::CREATED,
                Json(json!({ "inserted_id": inserted_id })),
            )),
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )),
        }
    }
}

// This is the handler function to be used in the route
#[debug_handler]
pub async fn insert_one_item(
    State(handler): State<Arc<ItemHandler>>,
    Json(req): Json<InserItemReq>,
) -> impl IntoResponse {
    handler.process_insert_one_item(req).await
}
