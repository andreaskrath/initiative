use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use super::repository::EncounterRepository;

pub async fn create(
    State(pool): State<PgPool>,
    Json(encounter): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Creating new encounter");
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.create(encounter).await {
        Ok(id) => {
            info!("Created encounter with id: {}", id);
            Ok(Json(serde_json::json!({ "id": id.to_string() })))
        }
        Err(err) => {
            error!("Failed to create encounter: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(encounter): Json<serde_json::Value>,
) -> StatusCode {
    info!("Updating encounter with id: {}", id);
    let encounter_repository = EncounterRepository::new(pool);

    if let Err(err) = encounter_repository.update(id, encounter).await {
        error!("Failed to update encounter {}: {:?}", id, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully updated encounter: {}", id);
    StatusCode::OK
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    info!("Fetching all encounters");
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.get_all().await {
        Ok(encounters) => {
            info!("Found {} encounters", encounters.len());
            Ok(Json(encounters))
        }
        Err(err) => {
            error!("Failed to fetch encounters: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("Fetching encounter with id: {}", id);
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.get_by_id(id).await {
        Ok(encounter) => {
            info!("Found encounter: {}", id);
            Ok(Json(encounter))
        }
        Err(err) => {
            error!("Failed to fetch encounter {}: {:?}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    info!("Deleting encounter with id: {}", id);
    let encounter_repository = EncounterRepository::new(pool);

    if let Err(err) = encounter_repository.delete(id).await {
        error!("Failed to delete encounter {}: {:?}", id, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully deleted encounter: {}", id);
    StatusCode::NO_CONTENT
}
