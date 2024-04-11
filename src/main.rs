mod db;
mod enums;
mod handlers;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use dotenv::dotenv;
use log::{info, LevelFilter};
use routes::routes::configure_routes;
use crate::db::get_database;
use actix_files::Files;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();

    let host = std::env::var("HOST").expect("HOST must be set in .env");
    let port = std::env::var("PORT").expect("PORT must be set in .env");

    info!("Запуск серверу!");

    let collection = get_database().await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allowed_methods(vec!["DELETE", "POST", "GET"]))
            .wrap(Logger::default())
            .service(Files::new("/download", "uploads/").show_files_listing())
            .app_data(web::Data::new(collection.clone()))
            .configure(configure_routes)
    });

    server.bind(format!("{}:{}", host, port))?
        .run()
        .await
        .expect("Помилка при запуску серверу!!!");

    Ok(())
}
