import { MagicSchool } from "$types/MagicSchool";
import { SpellLevel } from "$types/SpellLevel";
import { Class, SpellcastingClasses } from "$types/Class";
import { RecordFactory } from "$utils/factories";

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
  classes: Record<Class, boolean>;
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
    classes: RecordFactory(SpellcastingClasses, false),
    description: undefined,
    atHigherLevels: undefined,
  }),
};
