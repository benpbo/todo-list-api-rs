use std::collections::HashMap;

use crate::{
    application::TaskRepository,
    domain::{Task, TaskId},
};
use anyhow::Result;

pub struct InMemoryTaskRepository {
    tasks_by_id: HashMap<TaskId, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks_by_id: HashMap::new(),
        }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn get_task_by_id(&self, id: &TaskId) -> Result<Option<Task>> {
        Ok(self.tasks_by_id.get(id).cloned())
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>> {
        Ok(self.tasks_by_id.values().cloned().collect())
    }

    fn add_task(&mut self, task: &Task) -> Result<()> {
        self.tasks_by_id.insert(task.id.clone(), task.clone());
        Ok(())
    }
}
