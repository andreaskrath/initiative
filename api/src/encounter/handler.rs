use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use super::repository::EncounterRepository;

pub async fn create(
    State(pool): State<PgPool>,
    Json(encounter): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.create(encounter).await {
        Ok(id) => Ok(Json(serde_json::json!({ "id": id.to_string() }))),
        Err(err) => {
            println!("{err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(encounter): Json<serde_json::Value>,
) -> StatusCode {
    let encounter_repository = EncounterRepository::new(pool);

    if let Err(err) = encounter_repository.update(id, encounter).await {
        println!("{err:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.get_all().await {
        Ok(encounters) => Ok(Json(encounters)),
        Err(err) => {
            println!("{err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let encounter_repository = EncounterRepository::new(pool);

    match encounter_repository.get_by_id(id).await {
        Ok(encounter) => Ok(Json(encounter)),
        Err(err) => {
            println!("{err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
