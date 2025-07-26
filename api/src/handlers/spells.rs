use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::{repositories::spell::SpellRepository, types::spell::Spell};

pub async fn create(State(pool): State<PgPool>, Json(spell): Json<Spell>) -> StatusCode {
    let spell_repo = SpellRepository::new(pool);

    if let Err(err) = spell_repo.create(spell).await {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
