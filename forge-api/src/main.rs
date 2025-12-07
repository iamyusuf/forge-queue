use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};

use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/jobs", post(create_job));
    // .route("/jobs/:id/status", get(job_status));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Serialize)]
struct CreateJobResponse {
    id: String,
    status: String,
    queue_at: String,
    delay_seconds: u32,
    max_retries: u32,
    priority: u32,
}

async fn create_job() -> (StatusCode, Json<CreateJobResponse>) {
    let response = CreateJobResponse {
        id: "123".to_string(),
        status: "pending".to_string(),
        queue_at: "2023-01-01T00:00:00Z".to_string(),
        delay_seconds: 10,
        max_retries: 3,
        priority: 5,
    };

    (StatusCode::CREATED, Json(response))
}

#[allow(dead_code)]
async fn job_status() {
    // Implementation for getting job status
    todo!("implement job status")
}
