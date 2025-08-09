import { Class, MagicSchool, SpellLevel } from "$types";

export type Spell = {
  id?: string;
  name?: string;
  school?: MagicSchool;
  level?: SpellLevel;
  casting_time?: string;
  verbal: boolean;
  somatic: boolean;
  material?: string;
  material_consumed: boolean;
  ritual: boolean;
  concentration: boolean;
  duration?: string;
  range?: string;
  area?: string;
  shape?: string;
  classes: Class[];
  description?: string;
  at_higher_levels?: string;
};

export const SpellActions = {
  EmptySpell: (): Spell => ({
    id: undefined,
    name: undefined,
    school: undefined,
    level: undefined,
    casting_time: undefined,
    verbal: false,
    somatic: false,
    material: undefined,
    material_consumed: false,
    ritual: false,
    concentration: false,
    duration: undefined,
    range: undefined,
    area: undefined,
    shape: undefined,
    classes: [],
    description: undefined,
    at_higher_levels: undefined,
  }),
};
