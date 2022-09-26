use anyhow::Result;
use async_trait::async_trait;

use super::commands::UpdateTaskError;
use crate::domain::{Task, TaskId};

pub enum TaskRepositoryUpdateError {
    ItemNotFound,
    UnknownError(anyhow::Error),
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_task_by_id(&self, id: &TaskId) -> Result<Option<Task>>;
    async fn get_all_tasks(&self) -> Result<Vec<Task>>;
    async fn add_task(&self, task: &Task) -> Result<()>;
    async fn update_task(&self, task: &Task) -> Result<(), TaskRepositoryUpdateError>;
}

impl From<TaskRepositoryUpdateError> for UpdateTaskError {
    fn from(error: TaskRepositoryUpdateError) -> Self {
        match error {
            TaskRepositoryUpdateError::ItemNotFound => UpdateTaskError::TaskDoesNotExist,
            TaskRepositoryUpdateError::UnknownError(error) => UpdateTaskError::UnknownError(error),
        }
    }
}
