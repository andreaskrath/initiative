use sqlx::PgPool;
use tracing::{debug, instrument};
use uuid::Uuid;

pub struct EncounterRepository {
    pool: PgPool,
}

impl EncounterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self, encounter), fields(encounter_id))]
    pub async fn create(&self, mut encounter: serde_json::Value) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        debug!("Creating encounter with id: {}", id);

        // Inject the database ID into the state JSON
        if let Some(obj) = encounter.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id.to_string()));
        }

        sqlx::query("INSERT INTO encounters (id, state) VALUES ($1, $2);")
            .bind(id)
            .bind(encounter)
            .execute(&self.pool)
            .await?;

        debug!("Successfully inserted encounter into database");
        Ok(id)
    }

    #[instrument(skip(self, encounter))]
    pub async fn update(&self, id: Uuid, mut encounter: serde_json::Value) -> Result<(), sqlx::Error> {
        debug!("Updating encounter in database");
        // Ensure the database ID is in the state JSON
        if let Some(obj) = encounter.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id.to_string()));
        }

        sqlx::query(
            r#"
            UPDATE encounters SET state = $1
            WHERE id = $2;
            "#,
        )
        .bind(encounter)
        .bind(id)
        .execute(&self.pool)
        .await?;

        debug!("Successfully updated encounter in database");
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get_all(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
        debug!("Querying all encounters from database");
        let rows = sqlx::query_as::<_, (Uuid, serde_json::Value)>(
            "SELECT id, state FROM encounters;"
        )
        .fetch_all(&self.pool)
        .await?;

        // Inject the database ID into each state
        let results = rows.into_iter().map(|(id, mut state)| {
            if let Some(obj) = state.as_object_mut() {
                obj.insert("id".to_string(), serde_json::json!(id.to_string()));
            }
            state
        }).collect();

        debug!("Successfully retrieved all encounters from database");
        Ok(results)
    }

    #[instrument(skip(self))]
    pub async fn get_by_id(&self, id: Uuid) -> Result<serde_json::Value, sqlx::Error> {
        debug!("Querying encounter from database");
        let (db_id, mut state) = sqlx::query_as::<_, (Uuid, serde_json::Value)>(
            "SELECT id, state FROM encounters WHERE id = $1;"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        // Inject the database ID into the state
        if let Some(obj) = state.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(db_id.to_string()));
        }

        debug!("Successfully retrieved encounter from database");
        Ok(state)
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        debug!("Deleting encounter from database");
        sqlx::query("DELETE FROM encounters WHERE id = $1;")
            .bind(id)
            .execute(&self.pool)
            .await?;

        debug!("Successfully deleted encounter from database");
        Ok(())
    }
}
