use super::{
    commands::CreateTaskWithDescriptionCommand,
    queries::{GetAllTasksQuery, GetTaskByIdQuery},
    TaskRepository,
};
use crate::domain::{Task, TaskId};
use async_trait::async_trait;

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
        self.repository.get_all_tasks()
    }
}

#[async_trait]
impl<R: TaskRepository> GetTaskByIdQuery for TaskService<R> {
    async fn execute(&self, id: TaskId) -> anyhow::Result<Option<Task>> {
        self.repository.get_task_by_id(&id)
    }
}

#[async_trait]
impl<R: TaskRepository> CreateTaskWithDescriptionCommand for TaskService<R> {
    async fn execute(&mut self, description: String) -> anyhow::Result<Task> {
        let new_task = Task::new(TaskId::new(), description, false);
        self.repository.add_task(&new_task)?;

        Ok(new_task)
    }
}
