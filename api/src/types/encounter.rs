use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::JsonValue};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Encounter {
    pub id: Option<Uuid>,
    pub name: String,
    pub entities: JsonValue,
    pub active: i16,
}
