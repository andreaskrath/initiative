use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "movement_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Movement {
    Normal,
    Burrow,
    Climb,
    Fly,
    FlyHover,
    Swim,
}
