import { MagicSchool } from "$types/MagicSchool";
import { SpellLevel } from "$types/SpellLevel";
import { Class, SpellcastingClasses } from "$types/Class";
import { RecordFactory } from "$utils/factories";

export class Spell {
  name: string | null;
  school: MagicSchool | null;
  level: SpellLevel | null;
  castingTime: string | null;
  verbal: boolean;
  somatic: boolean;
  material: string | null;
  ritual: boolean;
  concentration: boolean;
  duration: string | null;
  range: string | null;
  area: string | null;
  areaType: string | null;
  classes: Record<Class, boolean>;

  constructor() {
    this.name = $state(null);
    this.school = $state(null);
    this.level = $state(null);
    this.castingTime = $state(null);
    this.verbal = $state(false);
    this.somatic = $state(false);
    this.material = $state(null);
    this.ritual = $state(false);
    this.concentration = $state(false);
    this.duration = $state(null);
    this.range = $state(null);
    this.area = $state(null);
    this.areaType = $state(null);
    this.classes = $state(RecordFactory(SpellcastingClasses, false));
  }
}
