use crate::domain::Task;
use anyhow::Result;

pub trait TaskRepository {
    fn get_all_tasks(&self) -> Result<Vec<Task>>;
}
