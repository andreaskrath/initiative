use crate::types::Monster;
use sqlx::{PgPool, query_as};

pub struct MonsterRepository {
    pool: PgPool,
}

impl MonsterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Box<[Monster]>, sqlx::Error> {
        let monsters: Vec<Monster> = query_as("SELECT * FROM v_monsters;")
            .fetch_all(&self.pool)
            .await?;

        Ok(monsters.into_boxed_slice())
    }
}
