use super::repository::MonsterRepository;
use crate::types::Monster;
use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(State(pool): State<PgPool>, Json(monster): Json<Monster>) -> StatusCode {
    let monster_repo = MonsterRepository::new(pool);

    if let Err(err) = monster_repo.create(monster).await {
        println!("{err:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    match MonsterRepository::new(pool).get_all().await {
        Ok(monsters) => Json(monsters).into_response(),
        Err(err) => {
            println!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Monster>, StatusCode> {
    let monster_repo = MonsterRepository::new(pool);

    match monster_repo.get_by_id(id).await {
        Ok(monster) => Ok(Json(monster)),
        Err(err) => {
            println!("{err:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let monster_repo = MonsterRepository::new(pool);

    if let Err(err) = monster_repo.delete(id).await {
        println!("{err:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::NO_CONTENT
}
