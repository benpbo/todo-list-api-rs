use super::TaskRepository;

pub struct TaskService<R: TaskRepository> {
    pub(crate) repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}
