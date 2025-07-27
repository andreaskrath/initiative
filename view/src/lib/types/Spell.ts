import { MagicSchool } from "$types/MagicSchool";
import { SpellLevel } from "$types/SpellLevel";
import { Class } from "$types/Class";

export type Spell = {
  name?: string;
  school?: MagicSchool;
  level?: SpellLevel;
  castingTime?: string;
  verbal: boolean;
  somatic: boolean;
  material?: string;
  materialConsumed: boolean;
  ritual: boolean;
  concentration: boolean;
  duration?: string;
  range?: string;
  area?: string;
  shape?: string;
  classes: Class[];
  description?: string;
  atHigherLevels?: string;
};

export const SpellActions = {
  EmptySpell: (): Spell => ({
    name: undefined,
    school: undefined,
    level: undefined,
    castingTime: undefined,
    verbal: false,
    somatic: false,
    material: undefined,
    materialConsumed: false,
    ritual: false,
    concentration: false,
    duration: undefined,
    range: undefined,
    area: undefined,
    shape: undefined,
    classes: [],
    description: undefined,
    atHigherLevels: undefined,
  }),
};
