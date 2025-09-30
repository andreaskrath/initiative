import { PrepareForValidation } from "$utils/validate";
import type { Encounter } from "$types";
import { ValidateEncounter } from "./validate";
import { CreateFieldErrors, type FieldErrors } from "$utils/error";

export const EncounterService = {
  Create: async (encounter: Encounter): Promise<number | FieldErrors> => {
    const preparedEncounter = PrepareForValidation(encounter);
    let errors = await ValidateEncounter(preparedEncounter);

    if (errors !== null) {
      return CreateFieldErrors(errors);
    }

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
