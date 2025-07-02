export enum Condition {
  Blinded = "Blinded",
  Charmed = "Charmed",
  Deafened = "Deafened",
  Frightened = "Frightened",
  Grappled = "Grappled",
  Incapacitated = "Incapacitated",
  Invisible = "Invisible",
  Paralyzed = "Paralyzed",
  Petrified = "Petrified",
  Poisoned = "Poisoned",
  Prone = "Prone",
  Restrained = "Restrained",
  Stunned = "Stunned",
  Unconscious = "Unconscious",
  Exhaustion = "Exhaustion",
}

export const Conditions: Condition[] = Object.values(Condition);
