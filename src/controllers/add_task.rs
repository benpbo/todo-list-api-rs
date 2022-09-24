use crate::application::{TaskRepository, TaskService};
use actix_web::{
    http::header::LOCATION,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::sync::Mutex;

pub async fn add_task<R: TaskRepository>(
    body: Json<AddTaskBody>,
    task_service: Data<Mutex<TaskService<R>>>,
) -> impl Responder {
    let AddTaskBody { description } = body.into_inner();

    match task_service.lock().unwrap().create_task(description).await {
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
