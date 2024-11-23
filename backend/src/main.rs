mod constants;
mod database;
mod models;
mod routes;
mod scrapers;
mod services;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .service(routes::scrape::scrape)
            .service(routes::draft::draft_player)
            .service(routes::draft::undraft_player)
            .service(routes::draft::get_player)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
