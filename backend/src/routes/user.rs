use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, put, web, HttpResponse, Result};

use crate::database::operations::user_operations;
use crate::models::user::{CreateUserRequest, UpdateUserRequest};

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
