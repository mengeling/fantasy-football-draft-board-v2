mod constants;
mod database;
mod models;
mod routes;
mod scrapers;
mod services;

use actix_web::{App, HttpServer};
use database::setup::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    if let Err(e) = init_db().await {
        eprintln!("Failed to initialize database: {}", e);
        return Ok(());
    }

    HttpServer::new(move || {
        App::new()
            .service(routes::scrape::scrape)
            .service(routes::draft::draft_player)
            .service(routes::draft::undraft_player)
            .service(routes::draft::get_player_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
