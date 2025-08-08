#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "damage_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    NonMagicalBludgeoning,
    NonMagicalPiercing,
    NonMagicalSlashing,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}
