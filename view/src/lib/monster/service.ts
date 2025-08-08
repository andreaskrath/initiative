import { type Monster } from "$types";
import { Validate } from "$monster/validate";

export const MonsterService = {
  Create: async (monster: Monster): Promise<number | string[]> => {
    let errors = await Validate(monster);
    if (errors.length > 0) {
      return errors;
    }

    const result = await fetch("/api/monsters/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(monster),
    });

    return result.status;
  },
};
