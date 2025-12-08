use serde::Deserialize;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub job_type: String,
    pub payload: Value,
    pub delay_seconds: u32,
    pub max_retries: u32,
    pub priority: u32,
    pub metadata: Value,
}

impl CreateJobRequest {
    #[allow(dead_code)]
    pub fn validate(&self) -> bool {
        // Example validation logic
        !self.job_type.is_empty() && self.delay_seconds <= 3600 && self.max_retries <= 10
    }

    #[allow(dead_code)]
    fn is_valid_max_retries(&self) -> bool {
        self.max_retries <= 10
    }

    #[allow(dead_code)]
    fn is_valid_delay(&self) -> bool {
        self.delay_seconds <= 3600
    }

    #[allow(dead_code)]
    fn is_valid_priority(&self) -> bool {
        self.priority <= 10
    }
}
