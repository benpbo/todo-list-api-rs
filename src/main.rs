mod application;
mod controllers;
mod domain;
mod infrastructure;

use actix_web::{
    middleware::Logger,
    web::{get, post, resource, Data, ServiceConfig},
    App, HttpServer,
};
use application::{commands::*, queries::*, TaskService};
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
            .configure(|cfg| config(cfg, task_service.clone()))
    };

    let address = ("0.0.0.0", 8080);
    HttpServer::new(app).bind(address)?.run().await?;

    Ok(())
}

fn config<S: GetAllTasksQuery + GetTaskByIdQuery + CreateTaskWithDescriptionCommand + 'static>(
    cfg: &mut ServiceConfig,
    task_service: Data<Mutex<S>>,
) {
    cfg.app_data(task_service)
        .service(
            resource("/tasks")
                .route(get().to(get_tasks::<S>))
                .route(post().to(add_task::<S>)),
        )
        .service(resource("/tasks/{id}").route(get().to(get_task::<S>)));
}
