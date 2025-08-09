export enum Class {
  Artificer = "artificer",
  Barbarian = "barbarian",
  Bard = "bard",
  Cleric = "cleric",
  Druid = "druid",
  Fighter = "fighter",
  Monk = "monk",
  Paladin = "paladin",
  Ranger = "ranger",
  Rogue = "rogue",
  Sorcerer = "sorcerer",
  Warlock = "warlock",
  Wizard = "wizard",
}

export const DisplayClass: Readonly<Record<Class, string>> = {
  [Class.Artificer]: "Artificer",
  [Class.Barbarian]: "Barbarian",
  [Class.Bard]: "Bard",
  [Class.Cleric]: "Cleric",
  [Class.Druid]: "Druid",
  [Class.Fighter]: "Fighter",
  [Class.Monk]: "Monk",
  [Class.Paladin]: "Paladin",
  [Class.Ranger]: "Ranger",
  [Class.Rogue]: "Rogue",
  [Class.Sorcerer]: "Sorcerer",
  [Class.Warlock]: "Warlock",
  [Class.Wizard]: "Wizard",
} as const;

export const Classes: Class[] = Object.values(Class);

export const SpellcastingClasses: Class[] = [
  Class.Artificer,
  Class.Bard,
  Class.Cleric,
  Class.Druid,
  Class.Paladin,
  Class.Ranger,
  Class.Sorcerer,
  Class.Warlock,
  Class.Wizard,
];
