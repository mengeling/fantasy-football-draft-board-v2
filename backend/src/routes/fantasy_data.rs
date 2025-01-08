use crate::database::operations::fantasy_data_operations;
use crate::services::fantasy_data_service;
use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Result};
use serde_json::json;

#[get("/fantasy-data/update")]
pub async fn update_fantasy_data() -> HttpResponse {
    match fantasy_data_service::update().await {
        Ok(_) => HttpResponse::Ok().json("Fantasy data update completed successfully"),
        Err(e) => {
            eprintln!("Fantasy data update failed: {}", e);
            HttpResponse::InternalServerError().json("Failed to update fantasy data")
        }
    }
}

#[get("/fantasy-data/last-update")]
pub async fn get_last_update() -> Result<HttpResponse> {
    let last_update = fantasy_data_operations::get_last_fantasy_data_update()
        .await
        .map_err(|e| {
            eprintln!("Failed to get last fantasy data update: {}", e);
            ErrorInternalServerError(e)
        })?;

    match last_update {
        Some(timestamp) => Ok(HttpResponse::Ok().json(json!({ "last_update": timestamp }))),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
