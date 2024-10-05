use axum::{routing::post, Router};
use items::{repositories::ItemRepository, usecases::ItemUseCase, handlers};
use tower::ServiceBuilder;
use http_body::BodyExt;
use std::{sync::Arc, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use crate::config::database::DatabaseConnection;
use crate::config::mongo::MongoDbConnection;

mod config;
pub mod items;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Define CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST]);

    // Database connection setup
    let db_connection: Arc<dyn DatabaseConnection + Send + Sync> = Arc::new(
        MongoDbConnection::new("mongodb://root:password@localhost:27017".to_string()),
    );

    // Inject the repository, use case, and handler
    let connection = db_connection.connect().await; // Connect once at startup
    let item_repository = Arc::new(ItemRepository::new(connection.clone()));
    let item_use_case = Arc::new(ItemUseCase::new(item_repository.clone()));
    let item_handler = Arc::new(ItemHandler::new(item_use_case.clone()));

    // Build the Axum application
    let app = Router::new()
        .route("/item", post(handlers::insert_one_item)) // Use free function handler
        .with_state(item_handler.clone()) // Provide the shared state
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .map_response_body(|body| body.boxed()), // Use the boxed() method
        );

    // Define the address to run the application on
    let addr = SocketAddr::from(([0, 0, 0, 0], 49153));
    println!("Listening on http://{}", addr);

    // Run the Axum application
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
