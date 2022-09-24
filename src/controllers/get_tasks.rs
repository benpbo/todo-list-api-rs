use crate::application::queries::GetAllTasksQuery;
use actix_web::{web::Data, HttpResponse, Responder};
use std::sync::Mutex;

pub async fn get_tasks<Q: GetAllTasksQuery>(get_tasks: Data<Mutex<Q>>) -> impl Responder {
    match get_tasks.lock().unwrap().execute().await {
        Ok(tasks) => HttpResponse::Ok().json(&tasks),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
