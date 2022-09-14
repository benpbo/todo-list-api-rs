use crate::{application::TaskRepository, domain::Task};
use anyhow::Result;

pub struct InMemoryTaskRepository {
    tasks: Vec<Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn get_all_tasks(&self) -> Result<Vec<Task>> {
        Ok(self.tasks.to_vec())
    }
}
