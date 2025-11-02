use sqlx::PgPool;
use uuid::Uuid;

pub struct EncounterRepository {
    pool: PgPool,
}

impl EncounterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, mut encounter: serde_json::Value) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();

        // Inject the database ID into the state JSON
        if let Some(obj) = encounter.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id.to_string()));
        }

        sqlx::query("INSERT INTO encounters (id, state) VALUES ($1, $2);")
            .bind(id)
            .bind(encounter)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    pub async fn update(&self, id: Uuid, mut encounter: serde_json::Value) -> Result<(), sqlx::Error> {
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

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<serde_json::Value>, sqlx::Error> {
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

        Ok(results)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<serde_json::Value, sqlx::Error> {
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

        Ok(state)
    }
}
