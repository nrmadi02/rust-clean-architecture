use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware::Logger};
use log::info;
use utoipa::OpenApi;
use crate::infrastructure::get_db_pool;

mod domain;
mod errors;
mod use_case;
mod adapters;
mod infrastructure;
mod open_api;

use adapters::repository::PostgresUserRepository;
use crate::adapters::api::{create_user, get_user};
use crate::open_api::ApiDoc;
use crate::use_case::UserUseCase;
use utoipa_swagger_ui::{SwaggerUi};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = get_db_pool().await.expect("Failed to connect to the database");

    let user_repository: Arc<dyn domain::UserRepository + Send + Sync> = Arc::new(PostgresUserRepository::new(pool));
    let user_use_case = UserUseCase::new(user_repository);

    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(user_use_case.clone()))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json",  ApiDoc::openapi()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}

