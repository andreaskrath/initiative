import { type Spell } from "$types";
import { ValidateSpell } from "$spell/validate";
import { PrepareForValidation } from "$utils/validate";
import type { FieldErrors } from "$utils/error";

export const CreateSpell = async (
  spell: Spell,
): Promise<number | FieldErrors> => {
  const preparedSpell = PrepareForValidation(spell);
  let errors = await ValidateSpell(preparedSpell);

  if (errors !== null) {
    return errors;
  }

  const result = await fetch("/api/spells/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(preparedSpell),
  });

  return result.status;
};

export const GetAllSpells = async (): Promise<Spell[]> => {
  const response = await fetch("/api/spells", { method: "GET" });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const spells = (await response.json()) as Spell[];
  return spells;
};
