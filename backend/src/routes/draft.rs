use actix_web::error::ErrorInternalServerError;
use actix_web::{delete, post, web, HttpRequest, HttpResponse, Result};

use crate::database::operations::draft_operations;
use crate::routes::utils::get_user_id;

#[post("/draft/{player_id}")]
pub async fn draft_player(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let drafted_player = draft_operations::draft_player(user_id, player_id.into_inner())
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
    let success = draft_operations::undraft_player(user_id, player_id.into_inner())
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
