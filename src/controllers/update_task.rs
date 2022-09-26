use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, HttpResponseBuilder, Responder,
};
use serde::Deserialize;

use crate::application::commands::{UpdateTaskCommand, UpdateTaskError};
use crate::domain::{Task, TaskId};

pub async fn update_task<C: UpdateTaskCommand>(
    path: Path<UpdateTaskPath>,
    body: Json<UpdateTaskBody>,
    update_task: Data<C>,
) -> impl Responder {
    let UpdateTaskPath { id } = path.into_inner();
    let UpdateTaskBody {
        description,
        is_completed,
    } = body.into_inner();
    let task = Task::new(id, description, is_completed);

    match update_task.execute(&task).await {
        Ok(()) => HttpResponse::NoContent(),
        Err(error) => error.into(),
    }
}

#[derive(Deserialize)]
pub struct UpdateTaskPath {
    id: TaskId,
}

#[derive(Deserialize)]
pub struct UpdateTaskBody {
    description: String,
    is_completed: bool,
}

impl From<UpdateTaskError> for HttpResponseBuilder {
    fn from(error: UpdateTaskError) -> Self {
        match error {
            UpdateTaskError::TaskDoesNotExist => HttpResponse::Conflict(),
            UpdateTaskError::UnknownError(error) => {
                log::error!(
                    "An unexpected error occured while trying to update a task: {}",
                    error
                );
                HttpResponse::InternalServerError()
            }
        }
    }
}
