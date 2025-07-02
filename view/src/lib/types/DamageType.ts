export enum DamageType {
  Acid = "Acid",
  Bludgeoning = "Bludgeoning",
  Cold = "Cold",
  Fire = "Fire",
  Force = "Force",
  Lightning = "Lightning",
  Necrotic = "Necrotic",
  NonMagicalBludgeoning = "Non-magical Bludgeoning",
  NonMagicalPiercing = "Non-magical Piercing",
  NonMagicalSlashing = "Non-magical Slashing",
  Piercing = "Piercing",
  Poison = "Poison",
  Psychic = "Psychic",
  Radiant = "Radiant",
  Slashing = "Slashing",
  Thunder = "Thunder",
}

export const DamageTypes: DamageType[] = Object.values(DamageType);
