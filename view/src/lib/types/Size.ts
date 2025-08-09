export enum Size {
  Tiny = "tiny",
  Small = "small",
  Medium = "medium",
  Large = "large",
  Huge = "huge",
  Gargantuan = "gargantuan",
}

export const DisplaySize: Readonly<Record<Size, string>> = {
  [Size.Tiny]: "Tiny",
  [Size.Small]: "Small",
  [Size.Medium]: "Medium",
  [Size.Large]: "Large",
  [Size.Huge]: "Huge",
  [Size.Gargantuan]: "Gargantuan",
} as const;

export const Sizes: Size[] = Object.values(Size);
