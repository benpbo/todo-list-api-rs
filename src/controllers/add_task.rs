use crate::application::{AddTaskCommand, TaskRepository, TaskService};
use actix::prelude::*;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub async fn add_task<R: TaskRepository>(
    body: Json<AddTaskBody>,
    task_service: Data<Addr<TaskService<R>>>,
) -> impl Responder {
    let AddTaskBody { description } = body.into_inner();
    let command = AddTaskCommand { description };

    let response = match task_service.send(command).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match response {
        Ok(new_task) => HttpResponse::Created()
            .append_header((LOCATION, format!("/tasks/{}", new_task.id)))
            .json(&new_task),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AddTaskBody {
    description: String,
}
