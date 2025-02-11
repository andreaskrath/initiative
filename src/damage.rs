pub const NEW_DAMAGE_TABLE: DamageTable = [false; 16];

pub type DamageTable = [bool; 16];

pub enum DamageType {
    NonMagicalBludgeoning,
    NonMagicalPiercing,
    NonMagicalSlashing,
    Bludgeoning,
    Piercing,
    Slashing,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}
