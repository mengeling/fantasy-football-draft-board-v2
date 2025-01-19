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
            .service(routes::drafted_players::draft_player)
            .service(routes::drafted_players::undraft_player)
            .service(routes::fantasy_data::get_last_update)
            .service(routes::fantasy_data::update_fantasy_data)
            .service(routes::players::get_players)
            .service(routes::users::create_user)
            .service(routes::users::get_user)
            .service(routes::users::update_user)
    })
    .bind((host, port))?
    .run()
    .await
}
