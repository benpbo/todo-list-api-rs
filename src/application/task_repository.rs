use crate::domain::{Task, TaskId};
use anyhow::Result;

pub trait TaskRepository: 'static + Unpin {
    fn get_task_by_id(&self, id: &TaskId) -> Result<Option<Task>>;
    fn get_all_tasks(&self) -> Result<Vec<Task>>;
    fn add_task(&mut self, task: &Task) -> Result<()>;
}
