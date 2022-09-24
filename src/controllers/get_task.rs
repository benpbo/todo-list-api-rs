use crate::{
    application::{TaskRepository, TaskService},
    domain::TaskId,
};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::sync::Mutex;

pub async fn get_task<R: TaskRepository>(
    task_service: Data<Mutex<TaskService<R>>>,
    path: Path<GetTaskByIdPath>,
) -> impl Responder {
    let GetTaskByIdPath { id } = path.into_inner();

    match task_service.lock().unwrap().get_task(id).await {
        Ok(Some(task)) => HttpResponse::Ok().json(&task),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[derive(Deserialize)]
pub struct GetTaskByIdPath {
    id: TaskId,
}
