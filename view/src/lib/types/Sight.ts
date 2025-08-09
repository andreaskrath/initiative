export enum Sight {
  Blindsight = "blindsight",
  Darkvision = "darkvision",
  Tremorsense = "tremorsense",
  Truesight = "truesight",
}

export const DisplaySight: Readonly<Record<Sight, string>> = {
  [Sight.Blindsight]: "Blindsight",
  [Sight.Darkvision]: "Darkvision",
  [Sight.Tremorsense]: "Tremorsense",
  [Sight.Truesight]: "Truesight",
} as const;

export const Sights: Sight[] = Object.values(Sight);
