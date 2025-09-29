import { PrepareForValidation } from "$utils/validate";
import type { Encounter } from "$types";

export const EncounterService = {
  Create: async (encounter: Encounter): Promise<number> => {
    const preparedEncounter = PrepareForValidation(encounter);
    // let errors = await ValidateSpell(preparedSpell);

    // if (errors !== null) {
    //   return errors;
    // }

    const result = await fetch("/api/encounters/create", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(preparedEncounter),
    });

    return result.status;
  },
};
