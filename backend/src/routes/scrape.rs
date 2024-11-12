use crate::services::scraper_service;
use actix_web::{get, HttpResponse};

#[get("/scrape")]
pub async fn scrape() -> HttpResponse {
    match scraper_service::run_scrapers().await {
        Ok(_) => HttpResponse::Ok().json("Scraping completed successfully"),
        Err(e) => {
            eprintln!("Scraping failed: {}", e);
            HttpResponse::InternalServerError().json("Failed to complete scraping")
        }
    }
}
