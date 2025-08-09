export enum Language {
  Abyssal = "abyssal",
  Celestial = "celestial",
  Common = "common",
  DeepSpeech = "deep_speech",
  Draconic = "draconic",
  Dwarvish = "dwarvish",
  Elvish = "elvish",
  Giant = "giant",
  Gnomish = "gnomish",
  Goblin = "goblin",
  Halfling = "halfling",
  Infernal = "infernal",
  Orc = "orc",
  Primordial = "primordial",
  Sylvan = "sylvan",
  Undercommon = "undercommon",
}

export const DisplayLanguage: Readonly<Record<Language, string>> = {
  [Language.Abyssal]: "Abyssal",
  [Language.Celestial]: "Celestial",
  [Language.Common]: "Common",
  [Language.DeepSpeech]: "Deep Speech",
  [Language.Draconic]: "Draconic",
  [Language.Dwarvish]: "Dwarvish",
  [Language.Elvish]: "Elvish",
  [Language.Giant]: "Giant",
  [Language.Gnomish]: "Gnomish",
  [Language.Goblin]: "Goblin",
  [Language.Halfling]: "Halfling",
  [Language.Infernal]: "Infernal",
  [Language.Orc]: "Orc",
  [Language.Primordial]: "Primordial",
  [Language.Sylvan]: "Sylvan",
  [Language.Undercommon]: "Undercommon",
} as const;

export const Languages: Language[] = Object.values(Language);
