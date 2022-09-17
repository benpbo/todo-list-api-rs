use crate::application::{GetAllTasksQuery, TaskRepository, TaskService};
use actix::prelude::*;
use actix_web::{web::Data, HttpResponse, Responder};

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
