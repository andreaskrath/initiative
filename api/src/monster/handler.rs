use super::repository::MonsterRepository;
use crate::types::Monster;
use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use tracing::{error, info};
use uuid::Uuid;

pub async fn create(State(pool): State<PgPool>, Json(monster): Json<Monster>) -> StatusCode {
    info!("Creating new monster: {}", monster.name);
    let monster_repo = MonsterRepository::new(pool);

    if let Err(err) = monster_repo.create(monster).await {
        error!("Failed to create monster: {:?}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully created monster");
    StatusCode::CREATED
}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    info!("Fetching all monsters");
    match MonsterRepository::new(pool).get_all().await {
        Ok(monsters) => {
            info!("Found {} monsters", monsters.len());
            Json(monsters).into_response()
        }
        Err(err) => {
            error!("Failed to fetch monsters: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Monster>, StatusCode> {
    info!("Fetching monster with id: {}", id);
    let monster_repo = MonsterRepository::new(pool);

    match monster_repo.get_by_id(id).await {
        Ok(monster) => {
            info!("Found monster: {}", monster.name);
            Ok(Json(monster))
        }
        Err(err) => {
            error!("Failed to fetch monster {}: {:?}", id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    info!("Deleting monster with id: {}", id);
    let monster_repo = MonsterRepository::new(pool);

    if let Err(err) = monster_repo.delete(id).await {
        error!("Failed to delete monster {}: {:?}", id, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    info!("Successfully deleted monster: {}", id);
    StatusCode::NO_CONTENT
}
