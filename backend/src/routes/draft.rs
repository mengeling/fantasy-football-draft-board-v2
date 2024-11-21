use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Result};

use crate::database::setup::DB_POOL;
use crate::models::requests::DraftRequest;
use crate::services::draft_service;

#[post("/draft")]
pub async fn draft_player(draft_req: web::Json<DraftRequest>) -> Result<HttpResponse> {
    let drafted_player =
        draft_service::draft_player(&*DB_POOL, draft_req.user_id, draft_req.player_id)
            .await
            .map_err(|e| {
                eprintln!("Failed to draft player: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;

    Ok(HttpResponse::Ok().json(drafted_player))
}

#[delete("/draft")]
pub async fn undraft_player(draft_req: web::Json<DraftRequest>) -> Result<HttpResponse> {
    let success = draft_service::undraft_player(&*DB_POOL, draft_req.user_id, draft_req.player_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to undraft player: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    if success {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/draft/player/{player_id}")]
pub async fn get_player_data(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = req
        .headers()
        .get("X-User-Id")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing X-User-Id header"))?
        .to_str()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid X-User-Id header format"))?
        .parse::<i32>()
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid X-User-Id header value"))?;

    let player_data = draft_service::get_player_data(&*DB_POOL, player_id.into_inner(), user_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get player data: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(player_data))
}
