use super::repository::SpellRepository;
use crate::types::Spell;
use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub async fn create(State(pool): State<PgPool>, Json(spell): Json<Spell>) -> StatusCode {
    info!("Creating new spell: {}", spell.name);
    let spell_repo = SpellRepository::new(pool);

    if let Err(err) = spell_repo.create(spell).await {
        error!("Failed to create spell: {:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully created spell");
    StatusCode::CREATED
}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    info!("Fetching all spells");
    match SpellRepository::new(pool).get_all().await {
        Ok(spells) => {
            info!("Found {} spells", spells.len());
            Json(spells).into_response()
        }
        Err(err) => {
            error!("Failed to fetch spells: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Spell>, StatusCode> {
    info!("Fetching spell with id: {}", id);
    let spell_repo = SpellRepository::new(pool);

    match spell_repo.get_by_id(id).await {
        Ok(spell) => {
            info!("Found spell: {}", spell.name);
            Ok(Json(spell))
        }
        Err(err) => {
            error!("Failed to fetch spell {}: {:?}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    info!("Deleting spell with id: {}", id);
    let spell_repo = SpellRepository::new(pool);

    if let Err(err) = spell_repo.delete(id).await {
        error!("Failed to delete spell {}: {:?}", id, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully deleted spell: {}", id);
    StatusCode::NO_CONTENT
}
