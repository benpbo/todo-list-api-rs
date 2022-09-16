use crate::{
    application::TaskRepository,
    domain::{Task, TaskId},
};
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
    fn get_task_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        Ok(self.tasks.iter().find(|task| task.id == id).cloned())
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>> {
        Ok(self.tasks.to_vec())
    }

    fn add_task(&mut self, task: &Task) -> Result<()> {
        self.tasks.push(task.clone());
        Ok(())
    }
}
