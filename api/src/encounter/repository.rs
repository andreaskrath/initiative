use sqlx::PgPool;
use uuid::Uuid;

use crate::types::Encounter;

pub struct EncounterRepository {
    pool: PgPool,
}

impl EncounterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, encounter: Encounter) -> Result<Uuid, sqlx::Error> {
        let encounter_id: sqlx::types::Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO encounters (
                name,
                entities,
                active
            ) VALUES ($1,$2,$3)
            RETURNING id;
            "#,
        )
        .bind(encounter.name)
        .bind(encounter.entities)
        .bind(encounter.active)
        .fetch_one(&self.pool)
        .await?;

        Ok(encounter_id)
    }
}
