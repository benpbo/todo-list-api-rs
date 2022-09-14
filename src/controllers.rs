use crate::application::GetAllTasks;
use actix_web::{web::Data, HttpResponse};

pub async fn get_tasks<S: GetAllTasks>(task_service: Data<S>) -> HttpResponse {
    match task_service.get_all_tasks() {
        Ok(tasks) => HttpResponse::Ok().json(&tasks),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
