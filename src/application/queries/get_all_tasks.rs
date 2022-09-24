use crate::domain::Task;
use async_trait::async_trait;

#[async_trait]
pub trait GetAllTasksQuery {
    async fn execute(&self) -> anyhow::Result<Vec<Task>, anyhow::Error>;
}
