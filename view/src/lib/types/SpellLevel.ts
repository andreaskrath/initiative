export enum SpellLevel {
  Cantrip = "cantrip",
  First = "first",
  Second = "second",
  Third = "third",
  Fourth = "fourth",
  Fifth = "fifth",
  Sixth = "sixth",
  Seventh = "seventh",
  Eighth = "eighth",
  Ninth = "ninth",
}

export const DisplaySpellLevel: Readonly<Record<SpellLevel, string>> = {
  [SpellLevel.Cantrip]: "Cantrip",
  [SpellLevel.First]: "First",
  [SpellLevel.Second]: "Second",
  [SpellLevel.Third]: "Third",
  [SpellLevel.Fourth]: "Fourth",
  [SpellLevel.Fifth]: "Fifth",
  [SpellLevel.Sixth]: "Sixth",
  [SpellLevel.Seventh]: "Seventh",
  [SpellLevel.Eighth]: "Eighth",
  [SpellLevel.Ninth]: "Ninth",
} as const;

export const SpellLevels: SpellLevel[] = Object.values(SpellLevel);
