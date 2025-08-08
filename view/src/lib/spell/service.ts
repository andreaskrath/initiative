import { type Spell } from "$types";
import { Validate } from "$spell/validate";

export const CreateSpell = async (spell: Spell): Promise<number | string[]> => {
  let errors = await Validate(spell);
  if (errors.length > 0) {
    return errors;
  }

  const result = await fetch("/api/spells/create", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(spell),
  });

  return result.status;
};

export const GetAllSpells = async (): Promise<Spell[]> => {
  const response = await fetch("/api/spells", { method: "GET" });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const data: Spell[] = await response.json();
  return data;
};
