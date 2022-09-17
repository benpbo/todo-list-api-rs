use super::super::{TaskRepository, TaskService};
use crate::domain::{Task, TaskId};
use actix::{Handler, Message};
use anyhow::Result;

#[derive(Message)]
#[rtype(result = "Result<Option<Task>>")]
pub struct GetTaskById {
    pub id: TaskId,
}

impl<R: TaskRepository> Handler<GetTaskById> for TaskService<R> {
    type Result = Result<Option<Task>>;

    fn handle(
        &mut self,
        GetTaskById { id }: GetTaskById,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.repository.get_task_by_id(&id)
    }
}
