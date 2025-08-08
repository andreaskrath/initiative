#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "sight", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Sight {
    Blindsight,
    Darkvision,
    Tremorsense,
    Truesight,
}
