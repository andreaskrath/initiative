use super::repository::MonsterRepository;
use crate::types::Monster;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

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
