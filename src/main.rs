use forge_core::job::Job;

fn main() {
    let job = Job {
        id: uuid::Uuid::new_v4(),
        job_type: "send_email".to_string(),
        payload: serde_json::json!({"key": "value"}),
        status: forge_core::job::JobStatus::Pending,
    };

    println!("Created job: {:?}", job);
}