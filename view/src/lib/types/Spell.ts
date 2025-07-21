import { MagicSchool } from "$types/MagicSchool";
import { SpellLevel } from "$types/SpellLevel";
import { Class, SpellcastingClasses } from "$types/Class";
import { RecordFactory } from "$utils/factories";

export type Spell = {
  name: string | null;
  school: MagicSchool | null;
  level: SpellLevel | null;
  castingTime: string | null;
  verbal: boolean;
  somatic: boolean;
  material: string | null;
  materialConsumed: boolean;
  ritual: boolean;
  concentration: boolean;
  duration: string | null;
  range: string | null;
  area: string | null;
  shape: string | null;
  classes: Record<Class, boolean>;
};

export const SpellActions = {
  EmptySpell: (): Spell => ({
    name: null,
    school: null,
    level: null,
    castingTime: null,
    verbal: false,
    somatic: false,
    material: null,
    materialConsumed: false,
    ritual: false,
    concentration: false,
    duration: null,
    range: null,
    area: null,
    shape: null,
    classes: RecordFactory(SpellcastingClasses, false),
  }),
};
