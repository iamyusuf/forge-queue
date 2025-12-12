use std::sync::Arc;

use axum::{Router, routing::{get, post}};

mod handler;
mod request;
mod response;
mod entity;

use handler::job::{create_job, job_status};
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://postgres:secret@localhost:5432/forge";
    let db = Database::connect(database_url).await.unwrap();
    let state = AppState { db };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/jobs", post(create_job))
        .route("/jobs/{id}/status", get(job_status))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}


