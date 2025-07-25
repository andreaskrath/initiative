import { type Spell } from "$types/Spell";

export const CreateSpell = async (spell: Spell): Promise<number | string[]> => {
  let errors = validateSpell(spell);
  if (errors.length > 0) {
    return errors;
  }

  const result = await fetch("/spells/create", {
    method: "POST",
    body: JSON.stringify(spell),
  });

  return result.status;
};

const validateSpell = (spell: Spell): string[] => {
  let errors: string[] = [];

  if (!spell.name) {
    errors.push("A name must be specified");
  }

  if (!spell.school) {
    errors.push("A school of magic must be specified");
  }

  if (!spell.level) {
    errors.push("A spell level must be specified");
  }

  if (!spell.castingTime) {
    errors.push("A casting time must be specified");
  }

  if (!spell.duration) {
    errors.push("A duration must be specified");
  }

  if (!spell.range) {
    errors.push("A range must be specified");
  }

  if (!spell.shape) {
    errors.push("A shape must be specified");
  }

  if (!spell.area) {
    errors.push("An area must be specified");
  }

  if (!spell.description) {
    errors.push("A description must be specified");
  }

  if (spell.materialConsumed && !spell.material) {
    errors.push("The material is consumed, but no material is specified");
  }

  if (Object.values(spell.classes).every((value) => !value)) {
    errors.push("No class restrictions have been specified");
  }

  return errors;
};
