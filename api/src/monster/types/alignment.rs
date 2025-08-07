use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "alignment", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Alignment {
    Any,
    ChaoticEvil,
    ChaoticGood,
    ChaoticNeutral,
    LawfulEvil,
    LawfulGood,
    LawfulNeutral,
    Neutral,
    NeutralEvil,
    NeutralGood,
    Unaligned,
}
