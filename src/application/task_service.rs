use super::TaskRepository;
use crate::domain::{Task, TaskCompleted, TaskId};

pub struct TaskService<R: TaskRepository> {
    pub(crate) repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_tasks(&self) -> anyhow::Result<Vec<Task>, anyhow::Error> {
        self.repository.get_all_tasks()
    }

    pub async fn get_task(&self, id: TaskId) -> anyhow::Result<Option<Task>> {
        self.repository.get_task_by_id(&id)
    }

    pub async fn create_task(&mut self, description: String) -> anyhow::Result<Task> {
        let new_task = Task::new(TaskId::new(), description, TaskCompleted::NotCompleted);
        self.repository.add_task(&new_task)?;

        Ok(new_task)
    }
}
