use axum::{Json, extract::Path, http::StatusCode};
use crate::response::{CreateJobResponse, JobStatusResponse};
use crate::request::CreateJobRequest;


pub async fn create_job(Json(request): Json<CreateJobRequest>) -> (StatusCode, Json<CreateJobResponse>) {
    println!("Received job creation request: {:?}", request);

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

pub async fn job_status(Path(id): Path<String>) -> (StatusCode, Json<JobStatusResponse>) {
    let response = JobStatusResponse {
        id,
        status: "in_progress".to_string(),
        started_at: Some("2023-01-01T00:10:00Z".to_string()),
        completed_at: None,
    };

    (StatusCode::OK, Json(response))
}
