export enum Attribute {
  Strength = "strength",
  Dexterity = "dexterity",
  Constitution = "constitution",
  Intelligence = "intelligence",
  Wisdom = "wisdom",
  Charisma = "charisma",
}

export const DisplayAttribute: Readonly<Record<Attribute, string>> = {
  [Attribute.Strength]: "Strength",
  [Attribute.Dexterity]: "Dexterity",
  [Attribute.Constitution]: "Constitution",
  [Attribute.Intelligence]: "Intelligence",
  [Attribute.Wisdom]: "Wisdom",
  [Attribute.Charisma]: "Charisma",
} as const;

export const Attributes: Attribute[] = Object.values(Attribute);
