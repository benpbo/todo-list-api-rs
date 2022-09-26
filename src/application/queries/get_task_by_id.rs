use crate::domain::{Task, TaskId};
use async_trait::async_trait;

#[async_trait]
pub trait GetTaskByIdQuery {
    async fn execute(&self, id: &TaskId) -> anyhow::Result<Option<Task>>;
}
