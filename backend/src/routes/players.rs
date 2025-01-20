use actix_web::error::ErrorInternalServerError;
use actix_web::{get, HttpRequest, HttpResponse, Result};

use crate::database::operations::player_operations;
use crate::routes::utils::get_user_id;

#[get("/players")]
pub async fn get_players(req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let players = player_operations::get_players(user_id).await.map_err(|e| {
        eprintln!("Failed to get players: {}", e);
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(players))
}
