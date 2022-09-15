use super::TaskRepository;
use actix::{Actor, Context};
use std::pin::Pin;

pub struct TaskService<R: TaskRepository> {
    pub(crate) repository: Pin<Box<R>>,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: Pin<Box<R>>) -> Self {
        Self { repository }
    }
}

impl<R: TaskRepository> Actor for TaskService<R> {
    type Context = Context<Self>;
}
