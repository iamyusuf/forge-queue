use async_trait::async_trait;
use forge_core::job::Job;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn enqueue(&self, job: Job) -> anyhow::Result<()>;
}

// Minimal in-memory storage for compilability
#[derive(Clone)]
pub struct InMemoryStorage;

#[async_trait]
impl Storage for InMemoryStorage {
    async fn enqueue(&self, _job: Job) -> anyhow::Result<()> {
        Ok(())
    }
}