use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Result};

use crate::constants::HEADER_USER_ID;
use crate::database::operations::{draft_operations, player_operations, user_operations};
use crate::models::player::PlayerQueryParameters;
use crate::models::user::{CreateUserRequest, UpdateUserRequest};

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

#[get("/user/{username}")]
pub async fn get_user(username: web::Path<String>) -> Result<HttpResponse> {
    let user = user_operations::get_user(&username).await.map_err(|e| {
        eprintln!("Failed to get user: {}", e);
        ErrorInternalServerError(e)
    })?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/user")]
pub async fn create_user(
    create_user_request: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    let new_user = user_operations::create_user(
        &create_user_request.username,
        &create_user_request.scoring_settings,
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to create user: {}", e);
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Created().json(new_user))
}

#[put("/user/{username}")]
pub async fn update_user(
    username: web::Path<String>,
    update_user_request: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    let updated_user =
        user_operations::update_user(&username, &update_user_request.scoring_settings)
            .await
            .map_err(|e| {
                eprintln!("Failed to update user: {}", e);
                ErrorInternalServerError(e)
            })?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("/player/{player_id}")]
pub async fn get_player(player_id: web::Path<i32>, req: HttpRequest) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;
    let player_data = player_operations::get_player(player_id.into_inner(), user_id)
        .await
        .map_err(|e| {
            eprintln!("Failed to get player data: {}", e);
            ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(player_data))
}

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
        query_parameters.drafted,
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to get players: {}", e);
        ErrorInternalServerError(e)
    })?;

    Ok(HttpResponse::Ok().json(players))
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
