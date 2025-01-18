use crate::models::rankings::ScoringSettings;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub scoring_settings: ScoringSettings,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "CreateUserRequestRaw")]
pub struct CreateUserRequest {
    pub username: String,
    pub scoring_settings: ScoringSettings,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequestRaw {
    username: String,
    scoring_settings: String,
}

impl TryFrom<CreateUserRequestRaw> for CreateUserRequest {
    type Error = actix_web::Error;

    fn try_from(raw: CreateUserRequestRaw) -> Result<Self, Self::Error> {
        println!("Attempting to parse scoring_settings: {}", raw.scoring_settings);
        let scoring_settings = raw.scoring_settings.parse().map_err(|e| {
            println!("Failed to parse scoring_settings: {:?}", e);
            actix_web::error::ErrorBadRequest("Invalid scoring_settings")
        })?;

        Ok(CreateUserRequest {
            username: raw.username,
            scoring_settings,
        })
    }
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub scoring_settings: ScoringSettings,
}
