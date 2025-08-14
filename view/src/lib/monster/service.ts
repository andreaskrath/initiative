import { type Monster } from "$types";
import { Validate } from "$monster/validate";
import { PrepareForValidation } from "$lib/shared/utils/validate";

export const MonsterService = {
  Create: async (monster: Monster): Promise<number | string[]> => {
    const preparedMonster = PrepareForValidation(monster);
    let errors = await Validate(preparedMonster);
    if (errors.length > 0) {
      return errors;
    }

    const cleanedMonster = cleanMonster(preparedMonster);

    const result = await fetch("/api/monsters/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(cleanedMonster),
    });

    return result.status;
  },
};

/*
 * Clean up a monster to prepare for backend transfer.
 */
const cleanMonster = (monster: Monster): Monster => {
  const { level, attribute, dc, attack_bonus, spell_slots, spells } =
    monster.spellcasting!;

  const hasContent =
    level !== undefined ||
    attribute !== undefined ||
    dc !== undefined ||
    attack_bonus !== undefined ||
    (spell_slots && spell_slots.length > 0) ||
    (spells && spells.length > 0);

  console.log(hasContent);
  if (!hasContent) {
    monster.spellcasting = undefined;
  }

  return monster;
};
