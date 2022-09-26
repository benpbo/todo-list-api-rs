use async_trait::async_trait;

use crate::domain::Task;

pub enum UpdateTaskError {
    TaskDoesNotExist,
    UnknownError(anyhow::Error),
}

#[async_trait]
pub trait UpdateTaskCommand {
    async fn execute(&self, task: &Task) -> Result<(), UpdateTaskError>;
}
