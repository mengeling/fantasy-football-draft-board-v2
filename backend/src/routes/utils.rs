use actix_web::error::ErrorBadRequest;
use actix_web::{HttpRequest, Result};

use crate::constants::HEADER_USER_ID;

pub fn get_user_id(req: &HttpRequest) -> Result<i32> {
    req.headers()
        .get(HEADER_USER_ID)
        .ok_or_else(|| ErrorBadRequest(format!("Missing {} header", HEADER_USER_ID)))?
        .to_str()
        .map_err(|_| ErrorBadRequest(format!("Invalid {} header format", HEADER_USER_ID)))?
        .parse::<i32>()
        .map_err(|_| ErrorBadRequest(format!("Invalid {} header value", HEADER_USER_ID)))
}
