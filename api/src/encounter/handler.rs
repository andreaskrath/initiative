use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::types::Encounter;

use super::repository::EncounterRepository;

pub async fn create(State(pool): State<PgPool>, Json(encounter): Json<Encounter>) -> StatusCode {
    let encounter_repository = EncounterRepository::new(pool);

    if let Err(err) = encounter_repository.create(encounter).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}
