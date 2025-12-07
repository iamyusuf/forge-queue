use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateJobResponse {
    pub id: String,
    pub status: String,
    pub queue_at: String,
    pub delay_seconds: u32,
    pub max_retries: u32,
    pub priority: u32,
}

#[derive(Debug, Serialize)]
pub struct JobStatusResponse {
    pub id: String,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}
