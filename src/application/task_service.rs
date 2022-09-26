use async_trait::async_trait;

use super::commands::{CreateTaskWithDescriptionCommand, UpdateTaskCommand, UpdateTaskError};
use super::queries::{GetAllTasksQuery, GetTaskByIdQuery};
use super::TaskRepository;
use crate::domain::{Task, TaskId};

pub struct TaskService<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: TaskRepository> GetAllTasksQuery for TaskService<R> {
    async fn execute(&self) -> anyhow::Result<Vec<Task>, anyhow::Error> {
        self.repository.get_all_tasks().await
    }
}

#[async_trait]
impl<R: TaskRepository> GetTaskByIdQuery for TaskService<R> {
    async fn execute(&self, id: &TaskId) -> anyhow::Result<Option<Task>> {
        self.repository.get_task_by_id(id).await
    }
}

#[async_trait]
impl<R: TaskRepository> CreateTaskWithDescriptionCommand for TaskService<R> {
    async fn execute(&self, description: String) -> anyhow::Result<Task> {
        let new_task = Task::new(TaskId::new(), description, false);
        self.repository.add_task(&new_task).await?;

        Ok(new_task)
    }
}

#[async_trait]
impl<R: TaskRepository> UpdateTaskCommand for TaskService<R> {
    async fn execute(&self, task: &Task) -> Result<(), UpdateTaskError> {
        Ok(self.repository.update_task(task).await?)
    }
}
