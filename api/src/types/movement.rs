#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "movement", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Movement {
    Normal,
    Burrow,
    Climb,
    Fly,
    FlyHover,
    Swim,
}
