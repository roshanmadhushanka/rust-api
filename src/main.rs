mod error;
mod handlers;
mod models;
mod state;

use axum::{routing::{get}, Router};
use tower_http::cors::CorsLayer;
use state::AppState;
use handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();

    let app = Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task).put(update_task).delete(delete_task))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await.unwrap();

    tracing::info!("Server is listening on http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}
