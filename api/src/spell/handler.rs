use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

use super::{repository::SpellRepository, types::Spell};

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
