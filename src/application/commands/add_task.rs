use super::super::{TaskRepository, TaskService};
use crate::domain::{Task, TaskId};
use actix::{Handler, Message};
use anyhow::Result;

#[derive(Message)]
#[rtype(result = "Result<Task>")]
pub struct AddTaskCommand {
    pub description: String,
}

impl<R: TaskRepository> Handler<AddTaskCommand> for TaskService<R> {
    type Result = Result<Task>;

    fn handle(
        &mut self,
        AddTaskCommand { description }: AddTaskCommand,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let new_task = Task {
            id: TaskId::new(),
            description,
            completed: false,
        };

        self.repository.add_task(&new_task)?;

        Ok(new_task)
    }
}
