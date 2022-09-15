mod application;
mod controllers;
mod domain;
mod infrastructure;

use actix::Actor;
use actix_web::{
    web::{get, Data},
    App, HttpServer,
};
use application::TaskService;
use controllers::get_tasks;
use infrastructure::InMemoryTaskRepository;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || {
        let task_repository = InMemoryTaskRepository::new();
        let task_service = TaskService::new(Box::pin(task_repository)).start();

        App::new()
            .app_data(Data::new(task_service))
            .route("/tasks", get().to(get_tasks::<InMemoryTaskRepository>))
    };

    let address = ("0.0.0.0", 8080);
    HttpServer::new(app).bind(address)?.run().await?;

    Ok(())
}
