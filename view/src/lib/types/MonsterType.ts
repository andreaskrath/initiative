export enum MonsterType {
  Aberration = "aberration",
  Beast = "beast",
  Celestial = "celestial",
  Construct = "construct",
  Dragon = "dragon",
  Elemental = "elemental",
  Fey = "fey",
  Fiend = "fiend",
  Giant = "giant",
  Humanoid = "humanoid",
  Monstrosity = "monstrosity",
  Ooze = "ooze",
  Plant = "plant",
  Undead = "undead",
}

export const DisplayMonsterType: Readonly<Record<MonsterType, string>> = {
  [MonsterType.Aberration]: "Aberration",
  [MonsterType.Beast]: "Beast",
  [MonsterType.Celestial]: "Celestial",
  [MonsterType.Construct]: "Construct",
  [MonsterType.Dragon]: "Dragon",
  [MonsterType.Elemental]: "Elemental",
  [MonsterType.Fey]: "Fey",
  [MonsterType.Fiend]: "Fiend",
  [MonsterType.Giant]: "Giant",
  [MonsterType.Humanoid]: "Humanoid",
  [MonsterType.Monstrosity]: "Monstrosity",
  [MonsterType.Ooze]: "Ooze",
  [MonsterType.Plant]: "Plant",
  [MonsterType.Undead]: "Undead",
} as const;

export const MonsterTypes: MonsterType[] = Object.values(MonsterType);
