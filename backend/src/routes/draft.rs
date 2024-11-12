use actix_web::{delete, post, web, HttpResponse, Result};

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
