use crate::application::commands::CreateTaskWithDescriptionCommand;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

pub async fn add_task<C: CreateTaskWithDescriptionCommand>(
    body: Json<AddTaskBody>,
    create_task: Data<C>,
) -> impl Responder {
    let AddTaskBody { description } = body.into_inner();

    match create_task.execute(description).await {
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
