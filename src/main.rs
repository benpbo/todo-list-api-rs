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
use aws_sdk_dynamodb::Client;
use controllers::{add_task, get_task, get_tasks};
use infrastructure::DynamoDbTaskRepository;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let sdk_config = aws_config::from_env().load().await;
    let task_repository = DynamoDbTaskRepository::new(Client::new(&sdk_config), "task");
    let task_service = Data::new(TaskService::new(task_repository));

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
    task_service: Data<S>,
) {
    cfg.app_data(task_service)
        .service(
            resource("/tasks")
                .route(get().to(get_tasks::<S>))
                .route(post().to(add_task::<S>)),
        )
        .service(resource("/tasks/{id}").route(get().to(get_task::<S>)));
}
