mod application;
mod controllers;
mod domain;
mod infrastructure;

use actix_web::{
    middleware::Logger,
    web::{get, post, resource, Data, ServiceConfig},
    App, HttpServer,
};
use application::TaskService;
use controllers::{add_task, get_task, get_tasks};
use infrastructure::InMemoryTaskRepository;
use std::{io, sync::Mutex};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let task_repository = InMemoryTaskRepository::new();
    let task_service = Data::new(Mutex::new(TaskService::new(task_repository)));

    let app = move || {
        App::new()
            .wrap(Logger::default())
            .app_data(task_service.clone())
            .configure(routes)
    };

    let address = ("0.0.0.0", 8080);
    HttpServer::new(app).bind(address)?.run().await?;

    Ok(())
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/tasks")
            .route(get().to(get_tasks::<InMemoryTaskRepository>))
            .route(post().to(add_task::<InMemoryTaskRepository>)),
    )
    .service(resource("/tasks/{id}").route(get().to(get_task::<InMemoryTaskRepository>)));
}
