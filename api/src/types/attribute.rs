#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "attribute", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
