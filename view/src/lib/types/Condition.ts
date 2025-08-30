export enum Condition {
  Blinded = "blinded",
  Charmed = "charmed",
  Deafened = "deafened",
  Frightened = "frightened",
  Grappled = "grappled",
  Incapacitated = "incapacitated",
  Invisible = "invisible",
  Paralyzed = "paralyzed",
  Petrified = "petrified",
  Poisoned = "poisoned",
  Prone = "prone",
  Restrained = "restrained",
  Stunned = "stunned",
  Unconscious = "unconscious",
  Exhaustion = "exhaustion",
}

export const DisplayCondition: Readonly<Record<Condition, string>> = {
  [Condition.Blinded]: "Blinded",
  [Condition.Charmed]: "Charmed",
  [Condition.Deafened]: "Deafened",
  [Condition.Frightened]: "Frightened",
  [Condition.Grappled]: "Grappled",
  [Condition.Incapacitated]: "Incapacitated",
  [Condition.Invisible]: "Invisible",
  [Condition.Paralyzed]: "Paralyzed",
  [Condition.Petrified]: "Petrified",
  [Condition.Poisoned]: "Poisoned",
  [Condition.Prone]: "Prone",
  [Condition.Restrained]: "Restrained",
  [Condition.Stunned]: "Stunned",
  [Condition.Unconscious]: "Unconscious",
  [Condition.Exhaustion]: "Exhaustion",
} as const;

export const Conditions: Condition[] = Object.values(Condition);

export const ConditionsNoExhaustion: Condition[] = Object.values(
  Condition,
).filter((condition) => condition !== Condition.Exhaustion);
