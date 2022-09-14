use super::{TaskRepository, TaskService};
use crate::domain::Task;
use anyhow::Result;

pub trait GetAllTasks {
    fn get_all_tasks(&self) -> Result<Vec<Task>>;
}

impl<R: TaskRepository> GetAllTasks for TaskService<R> {
    fn get_all_tasks(&self) -> Result<Vec<Task>> {
        self.repository.get_all_tasks()
    }
}
