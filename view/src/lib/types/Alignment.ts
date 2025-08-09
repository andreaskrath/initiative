export enum Alignment {
  Any = "any",
  ChaoticEvil = "chaotic_evil",
  ChaoticGood = "chaotic_good",
  ChaoticNeutral = "chaotic_neutral",
  LawfulEvil = "lawful_evil",
  LawfulGood = "lawful_good",
  LawfulNeutral = "lawful_neutral",
  Neutral = "neutral",
  NeutralEvil = "neutral_evil",
  NeutralGood = "neutral_good",
  Unaligned = "unaligned",
}

export const DisplayAlignment: Readonly<Record<Alignment, string>> = {
  [Alignment.Any]: "Any",
  [Alignment.ChaoticEvil]: "Chaotic Evil",
  [Alignment.ChaoticGood]: "Chaotic Good",
  [Alignment.ChaoticNeutral]: "Chaotic Neutral",
  [Alignment.LawfulEvil]: "Lawful Evil",
  [Alignment.LawfulGood]: "Lawful Good",
  [Alignment.LawfulNeutral]: "Lawful Neutral",
  [Alignment.Neutral]: "Neutral",
  [Alignment.NeutralEvil]: "Neutral Evil",
  [Alignment.NeutralGood]: "Neutral Good",
  [Alignment.Unaligned]: "Unaligned",
} as const;

export const Alignments: Alignment[] = Object.values(Alignment);
