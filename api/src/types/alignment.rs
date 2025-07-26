#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "alignment", rename_all = "lowercase")]
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
