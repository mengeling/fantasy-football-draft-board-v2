use actix_web::error::ErrorInternalServerError;
use actix_web::{get, web, HttpRequest, HttpResponse, Result};

use crate::database::operations::player_operations;
use crate::models::player::PlayerQueryParameters;
use crate::routes::utils::get_user_id;

#[get("/players")]
pub async fn get_players(
    query_parameters: web::Query<PlayerQueryParameters>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let players = player_operations::get_players(
        user_id,
        query_parameters.position.as_ref(),
        query_parameters.team.as_ref(),
        query_parameters.name.as_deref(),
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to get players: {}", e);
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(players))
}
