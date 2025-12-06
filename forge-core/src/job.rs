use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub job_type: String,
    pub payload: serde_json::Value,
    pub status: JobStatus,
}

impl Job {
    pub fn new() -> Self {
        Job {
            id: uuid::Uuid::new_v4(),
            job_type: "send_email".to_string(),
            payload: serde_json::json!({"key": "value"}),
            status: JobStatus::Pending,
        }
    }
}
