mod constants;
mod database;
mod models;
mod routes;
mod scrapers;
mod services;

use actix_web::{App, HttpServer};
use database::connection::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    init_pool()
        .await
        .expect("Failed to initialize database pool");

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .service(routes::scrape::scrape)
            .service(routes::draft::draft_player)
            .service(routes::draft::undraft_player)
            .service(routes::draft::get_user)
            .service(routes::draft::create_user)
            .service(routes::draft::update_user)
            .service(routes::draft::get_player)
            .service(routes::draft::get_players)
    })
    .bind((host, port))?
    .run()
    .await
}
