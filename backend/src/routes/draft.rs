use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Result};

use crate::constants::HEADER_USER_ID;
use crate::database::connection::DB_POOL;
use crate::services::draft_service;

#[post("/draft/{player_id}")]
pub async fn draft_player(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let drafted_player = draft_service::draft_player(&*DB_POOL, user_id, player_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("Failed to draft player: {}", e);
            ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(drafted_player))
}

#[delete("/draft/{player_id}")]
pub async fn undraft_player(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let success = draft_service::undraft_player(&*DB_POOL, user_id, player_id.into_inner())
        .await
        .map_err(|e| {
            eprintln!("Failed to undraft player: {}", e);
            ErrorInternalServerError(e)
        })?;

    if success {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[get("/draft/player/{player_id}")]
pub async fn get_player_data(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let player_data = draft_service::get_player_data(&*DB_POOL, player_id.into_inner(), user_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get player data: {}", e);
            ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(player_data))
}

fn get_user_id(req: &HttpRequest) -> Result<i32> {
    req.headers()
        .get(HEADER_USER_ID)
        .ok_or_else(|| ErrorBadRequest(format!("Missing {} header", HEADER_USER_ID)))?
        .to_str()
        .map_err(|_| ErrorBadRequest(format!("Invalid {} header format", HEADER_USER_ID)))?
        .parse::<i32>()
        .map_err(|_| ErrorBadRequest(format!("Invalid {} header value", HEADER_USER_ID)))
}
