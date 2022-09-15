use crate::domain::Task;
use anyhow::Result;

pub trait TaskRepository: 'static + Unpin {
    fn get_all_tasks(&self) -> Result<Vec<Task>>;
    fn add_task(&mut self, task: &Task) -> Result<()>;
}
