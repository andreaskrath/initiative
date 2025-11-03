use super::repository::SpellRepository;
use crate::types::Spell;
use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(State(pool): State<PgPool>, Json(spell): Json<Spell>) -> StatusCode {
    let spell_repo = SpellRepository::new(pool);

    if let Err(err) = spell_repo.create(spell).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::CREATED
}

pub async fn get(State(pool): State<PgPool>) -> impl IntoResponse {
    match SpellRepository::new(pool).get_all().await {
        Ok(spells) => Json(spells).into_response(),
        Err(err) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Spell>, StatusCode> {
    let spell_repo = SpellRepository::new(pool);

    match spell_repo.get_by_id(id).await {
        Ok(spell) => Ok(Json(spell)),
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
    let spell_repo = SpellRepository::new(pool);

    if let Err(err) = spell_repo.delete(id).await {
        println!("{err:?}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::NO_CONTENT
}
