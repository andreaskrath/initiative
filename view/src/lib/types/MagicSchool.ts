export enum MagicSchool {
  Abjuration = "abjuration",
  Conjuration = "conjuration",
  Divination = "divination",
  Enchantment = "enchantment",
  Evocation = "evocation",
  Illusion = "illusion",
  Necromancy = "necromancy",
  Transmutation = "transmutation",
}

export const DisplayMagicSchool: Readonly<Record<MagicSchool, string>> = {
  [MagicSchool.Abjuration]: "Abjuration",
  [MagicSchool.Conjuration]: "Conjuration",
  [MagicSchool.Divination]: "Divination",
  [MagicSchool.Enchantment]: "Enchantment",
  [MagicSchool.Evocation]: "Evocation",
  [MagicSchool.Illusion]: "Illusion",
  [MagicSchool.Necromancy]: "Necromancy",
  [MagicSchool.Transmutation]: "Transmutation",
} as const;

export const MagicSchools: MagicSchool[] = Object.values(MagicSchool);
