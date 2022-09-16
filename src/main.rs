mod application;
mod controllers;
mod domain;
mod infrastructure;

use actix::Actor;
use actix_web::{
    web::{get, post, Data},
    App, HttpServer,
};
use application::TaskService;
use controllers::{add_task, get_tasks};
use infrastructure::InMemoryTaskRepository;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let task_repository = InMemoryTaskRepository::new();
    let task_service = TaskService::new(task_repository).start();

    let app = move || {
        App::new()
            .app_data(Data::new(task_service.clone()))
            .route("/tasks", get().to(get_tasks::<InMemoryTaskRepository>))
            .route("/tasks", post().to(add_task::<InMemoryTaskRepository>))
    };

    let address = ("0.0.0.0", 8080);
    HttpServer::new(app).bind(address)?.run().await?;

    Ok(())
}
