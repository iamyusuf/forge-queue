use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
};

mod response;
use response::{CreateJobResponse, JobStatusResponse};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/jobs", post(create_job))
        .route("/jobs/{id}/status", get(job_status));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
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

async fn job_status(Path(id): Path<String>) -> (StatusCode, Json<JobStatusResponse>) {
    let response = JobStatusResponse {
        id,
        status: "in_progress".to_string(),
        started_at: Some("2023-01-01T00:10:00Z".to_string()),
        completed_at: None,
    };

    (StatusCode::OK, Json(response))
}
