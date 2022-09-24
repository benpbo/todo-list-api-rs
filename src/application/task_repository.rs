use crate::domain::{Task, TaskId};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_task_by_id(&self, id: &TaskId) -> Result<Option<Task>>;
    async fn get_all_tasks(&self) -> Result<Vec<Task>>;
    async fn add_task(&mut self, task: &Task) -> Result<()>;
}
