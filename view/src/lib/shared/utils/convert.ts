/** Get a modifier from an ability score. */
export const GetModifier = (score: number): number => {
  return Math.round((score - 10) / 2);
};
