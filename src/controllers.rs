use crate::application::{AddTaskCommand, GetAllTasksQuery, TaskRepository, TaskService};
use actix::prelude::*;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub async fn get_tasks<R: TaskRepository>(
    task_service: Data<Addr<TaskService<R>>>,
) -> impl Responder {
    let query = GetAllTasksQuery {};
    let response = match task_service.send(query).await {
        Ok(response) => response,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match response {
        Ok(tasks) => HttpResponse::Ok().json(&tasks),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

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
        Ok(new_task) => HttpResponse::Ok().json(&new_task),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AddTaskBody {
    description: String,
}
