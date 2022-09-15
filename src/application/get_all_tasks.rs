use super::{TaskRepository, TaskService};
use crate::domain::Task;
use actix::{Handler, Message};
use anyhow::Result;

#[derive(Message)]
#[rtype(result = "Result<Vec<Task>>")]
pub struct GetAllTasksQuery;

impl<R: TaskRepository> Handler<GetAllTasksQuery> for TaskService<R> {
    type Result = Result<Vec<Task>>;

    fn handle(&mut self, _msg: GetAllTasksQuery, _ctx: &mut Self::Context) -> Self::Result {
        self.repository.get_all_tasks()
    }
}
