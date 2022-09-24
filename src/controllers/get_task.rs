use crate::{application::queries::GetTaskByIdQuery, domain::TaskId};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;
use std::sync::Mutex;

pub async fn get_task<Q: GetTaskByIdQuery>(
    get_task: Data<Mutex<Q>>,
    path: Path<GetTaskByIdPath>,
) -> impl Responder {
    let GetTaskByIdPath { id } = path.into_inner();

    match get_task.lock().unwrap().execute(id).await {
        Ok(Some(task)) => HttpResponse::Ok().json(&task),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[derive(Deserialize)]
pub struct GetTaskByIdPath {
    id: TaskId,
}
