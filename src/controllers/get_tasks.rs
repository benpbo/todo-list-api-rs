use crate::application::{TaskRepository, TaskService};
use actix_web::{web::Data, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn get_tasks<R: TaskRepository>(
    task_service: Data<Mutex<TaskService<R>>>,
) -> impl Responder {
    match task_service.lock().unwrap().get_tasks().await {
        Ok(tasks) => HttpResponse::Ok().json(&tasks),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
