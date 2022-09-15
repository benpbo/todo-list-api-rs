use crate::domain::Task;
use anyhow::Result;

pub trait TaskRepository: 'static {
    fn get_all_tasks(&self) -> Result<Vec<Task>>;
}
