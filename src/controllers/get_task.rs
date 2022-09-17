use crate::{
    application::{GetTaskById, TaskRepository, TaskService},
    domain::TaskId,
};
use actix::prelude::*;
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub async fn get_task<R: TaskRepository>(
    task_service: Data<Addr<TaskService<R>>>,
    path: Path<GetTaskByIdPath>,
) -> impl Responder {
    let GetTaskByIdPath { id } = path.into_inner();
    let query = GetTaskById { id };

    let response = match task_service.send(query).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match response {
        Ok(Some(task)) => HttpResponse::Ok().json(&task),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[derive(Deserialize)]
pub struct GetTaskByIdPath {
    id: TaskId,
}
