use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "spell_level", rename_all = "lowercase")]
pub enum SpellLevel {
    Cantrip,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
}
