export enum Class {
  Artificer = "Artificer",
  Barbarian = "Barbarian",
  Bard = "Bard",
  Cleric = "Cleric",
  Druid = "Druid",
  Fighter = "Fighter",
  Monk = "Monk",
  Paladin = "Paladin",
  Ranger = "Ranger",
  Rogue = "Rogue",
  Sorcerer = "Sorcerer",
  Warlock = "Warlock",
  Wizard = "Wizard",
}

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
