export enum Range {
  Self = "Self",
  Touch = "Touch",
  Sight = "Sight",
  Feet30 = "30 ft.",
  Feet60 = "60 ft.",
  Feet90 = "90 ft.",
  Feet120 = "120 ft.",
  Feet150 = "150 ft.",
  Feet300 = "300 ft.",
  Feet500 = "500 ft.",
  Mile1 = "1 mile",
  Unlimited = "Unlimited",
}

export const Ranges: Range[] = Object.values(Range);
