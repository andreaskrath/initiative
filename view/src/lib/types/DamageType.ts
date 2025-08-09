export enum DamageType {
  Acid = "acid",
  Bludgeoning = "bludgeoning",
  Cold = "cold",
  Fire = "fire",
  Force = "force",
  Lightning = "lightning",
  Necrotic = "necrotic",
  NonMagicalBludgeoning = "non_magical_bludgeoning",
  NonMagicalPiercing = "non_magical_piercing",
  NonMagicalSlashing = "non_magical_slashing",
  Piercing = "piercing",
  Poison = "poison",
  Psychic = "psychic",
  Radiant = "radiant",
  Slashing = "slashing",
  Thunder = "thunder",
}

export const DisplayDamageType: Readonly<Record<DamageType, string>> = {
  [DamageType.Acid]: "Acid",
  [DamageType.Bludgeoning]: "Bludgeoning",
  [DamageType.Cold]: "Cold",
  [DamageType.Fire]: "Fire",
  [DamageType.Force]: "Force",
  [DamageType.Lightning]: "Lightning",
  [DamageType.Necrotic]: "Necrotic",
  [DamageType.NonMagicalBludgeoning]: "Non-magical Bludgeoning",
  [DamageType.NonMagicalPiercing]: "Non-magical Piercing",
  [DamageType.NonMagicalSlashing]: "Non-magical Slashing",
  [DamageType.Piercing]: "Piercing",
  [DamageType.Poison]: "Poison",
  [DamageType.Psychic]: "Psychic",
  [DamageType.Radiant]: "Radiant",
  [DamageType.Slashing]: "Slashing",
  [DamageType.Thunder]: "Thunder",
} as const;

export const DamageTypes: DamageType[] = Object.values(DamageType);
