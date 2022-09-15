use super::TaskRepository;
use actix::{Actor, Context};

pub struct TaskService<R: TaskRepository> {
    pub(crate) repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: TaskRepository> Actor for TaskService<R> {
    type Context = Context<Self>;
}
