use axum::{Router, routing::{get, post}};

mod handler;
mod request;
mod response;

use handler::job::{create_job, job_status};

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


