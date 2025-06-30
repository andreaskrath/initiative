<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import Dices from "@lucide/svelte/icons/dices";

  import { Alignment, Alignments } from "../types/Alignment";
  import { Attribute, Attributes } from "../types/Attribute";
  import { Condition, Conditions } from "../types/Condition";
  import { DamageType, DamageTypes } from "../types/DamageType";
  import { Language, Languages } from "../types/Language";
  import { MonsterType, MonsterTypes } from "../types/MonsterType";
  import { Sight, Sights } from "../types/Sight";
  import { Size, Sizes } from "../types/Size";
  import { Skill, Skills } from "../types/Skill";

  function AttributesFactory() {
    return Attributes.reduce(
      (record, attribute) => {
        record[attribute] = null;
        return record;
      },
      {} as Record<Attribute, number | null>,
    );
  }

  function DamageTypesFactory() {
    return DamageTypes.reduce(
      (record, damageType) => {
        record[damageType] = false;
        return record;
      },
      {} as Record<DamageType, boolean>,
    );
  }

  function LanguagesFactory() {
    return Languages.reduce(
      (record, language) => {
        record[language] = false;
        return record;
      },
      {} as Record<Language, boolean>,
    );
  }

  function SkillsFactory() {
    return Skills.reduce(
      (record, skill) => {
        record[skill] = null;
        return record;
      },
      {} as Record<Skill, number | null>,
    );
  }

  // Need to use a inline object, because classes are broken as reactive components.
  let monster = $state({
    name: null as string | null,
    challengeRating: null as number | null,
    xp: null as number | null,
    proficiencyBonus: null as number | null,
    size: Size.Medium,
    monsterType: MonsterType.Beast,
    species: null as string | null,
    alignment: Alignment.Any,
    attributes: AttributesFactory(),
    hitPoints: null as number | null,
    rollableHitPoints: null as string | null,
    armorClass: null as number | null,
    armorType: null as string | null,
    savingThrows: AttributesFactory(),
    damageResistances: DamageTypesFactory(),
    damageImmunities: DamageTypesFactory(),
    visions: [] as { type: Sight; range: number | null }[],
    languages: LanguagesFactory(),
    skills: SkillsFactory(),
    traits: [] as { name: string | null; description: string | null }[],
  });

  function addVision(event: MouseEvent) {
    monster.visions = [
      ...monster.visions,
      { type: Sight.Darkvision, range: null },
    ];
    event.preventDefault();
  }

  function removeVision(visionToRemove: { type: Sight; range: number | null }) {
    return function (event: MouseEvent) {
      monster.visions = monster.visions.filter(
        (vision) => vision !== visionToRemove,
      );
      event.preventDefault();
    };
  }

  function addTrait(event: MouseEvent) {
    monster.traits = [...monster.traits, { name: null, description: null }];

    event.preventDefault();
  }

  function removeTrait(traitToRemove: {
    name: string | null;
    description: string | null;
  }) {
    return function (event: MouseEvent) {
      monster.traits = monster.traits.filter(
        (trait) => trait !== traitToRemove,
      );
      event.preventDefault();
    };
  }

  // Comment in for console.logs of each state update.
  // $inspect(monster);
</script>

<div class="py-2"></div>
<form class="w-[1200px] mx-auto space-y-2 px-2">
  <!-- Basic Information -->
  <h2 class="h2">Basic Information</h2>

  <!-- Name, Challenge, XP & Proficiency-->
  <div class="input-group grid-cols-16">
    <!-- Name -->
    <div class="ig-cell preset-tonal col-span-1">Name</div>
    <input
      class="ig-input col-span-6"
      type="text"
      placeholder="Goblin"
      bind:value={monster.name}
    />

    <!-- Challenge Rating -->
    <div class="ig-cell preset-tonal col-span-2">Challenge</div>
    <input
      class="ig-input col-span-1 text-center no-spinner"
      type="number"
      placeholder="0.5"
      bind:value={monster.challengeRating}
    />

    <!-- Experience Points -->
    <div class="ig-cell preset-tonal col-span-1">XP</div>
    <input
      class="ig-input col-span-2 text-center"
      type="number"
      placeholder="100"
      bind:value={monster.xp}
    />

    <!-- Proficiency Bonus -->
    <div class="ig-cell preset-tonal col-span-2">Proficiency</div>
    <input
      class="ig-input col-span-1 text-center"
      type="number"
      placeholder="2"
      bind:value={monster.proficiencyBonus}
    />
  </div>

  <!-- Size, Type, Species & Alignment -->
  <div class="input-group grid-cols-32">
    <!-- Size -->
    <div class="ig-cell preset-tonal col-span-2">Size</div>
    <select class="ig-select col-span-5 text-center" bind:value={monster.size}>
      {#each Sizes as size}
        <option>{size}</option>
      {/each}
    </select>

    <!-- Monster Type -->
    <div class="ig-cell preset-tonal col-span-2">Type</div>
    <select
      class="ig-select col-span-6 text-center"
      bind:value={monster.monsterType}
    >
      {#each MonsterTypes as monsterType}
        <option>{monsterType}</option>
      {/each}
    </select>

    <!-- Species -->
    <div class="ig-cell preset-tonal col-span-2">Species</div>
    <input
      class="ig-input col-span-6"
      type="text"
      placeholder="Goblinoid"
      bind:value={monster.species}
    />

    <!-- Alignment -->
    <div class="ig-cell preset-tonal col-span-3">Alignment</div>
    <select
      class="ig-select col-span-6 text-center"
      bind:value={monster.alignment}
    >
      {#each Alignments as alignment}
        <option>{alignment}</option>
      {/each}
    </select>
  </div>

  <!-- Attributes -->
  <hr class="hr" />
  <h2 class="h2">Attributes</h2>

  <div class="input-group grid-cols-18">
    {#each Attributes as attribute}
      <div class="ig-cell preset-tonal col-span-2">{attribute}</div>
      <input
        class="ig-input text-center"
        type="number"
        placeholder="13"
        bind:value={monster.attributes[attribute]}
      />
    {/each}
  </div>

  <!-- Defensive -->
  <hr class="hr" />
  <h2 class="h2">Defensive</h2>

  <div class="input-group grid-cols-16">
    <!-- Hit Points -->
    <div class="ig-cell preset-tonal col-span-2">Hit Points</div>
    <input
      class="ig-input col-span-1 text-center"
      type="number"
      placeholder="11"
      bind:value={monster.hitPoints}
    />

    <!-- Rollable Hit Points -->
    <div class="ig-cell preset-tonal col-span-1" title="Rollable Hit Points">
      <Dices />
    </div>
    <input
      class="ig-input text-center col-span-2"
      type="text"
      placeholder="2d8 +2"
      bind:value={monster.rollableHitPoints}
    />

    <!-- Armor Class -->
    <div class="ig-cell preset-tonal col-span-2">Armor Class</div>
    <input
      class="ig-input text-center col-span-1"
      type="number"
      placeholder="18"
      bind:value={monster.armorClass}
    />

    <!-- Armor Type -->
    <div class="ig-cell preset-tonal col-span-2">Armor Type</div>
    <input
      class="ig-input col-span-5"
      type="text"
      placeholder="chain mail, shield"
      bind:value={monster.armorType}
    />
  </div>

  <!-- Saving Throws -->
  <h6 class="h6">Saving Throws</h6>
  <div class="input-group grid-cols-18">
    {#each Attributes as attribute}
      <div class="ig-cell preset-tonal col-span-2">{attribute}</div>
      <input
        class="ig-input text-center"
        type="number"
        bind:value={monster.savingThrows[attribute]}
      />
    {/each}
  </div>

  <!-- Damage Resistances -->
  <h6 class="h6">Damage Resistances</h6>
  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each DamageTypes as damageType}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input
          class="checkbox"
          type="checkbox"
          bind:checked={monster.damageResistances[damageType]}
        />
        <p>{damageType}</p>
      </label>
    {/each}
  </div>

  <!-- Damage Immunities -->
  <h6 class="h6">Damage Immunities</h6>
  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each DamageTypes as damageType}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input
          class="checkbox"
          type="checkbox"
          bind:checked={monster.damageImmunities[damageType]}
        />
        <p>{damageType}</p>
      </label>
    {/each}
  </div>

  <!-- Vision & Perception -->
  <hr class="hr" />
  <div class="flex justify-between">
    <h2 class="h2">Senses</h2>
    <button
      type="button"
      class="btn border-none text-success-300"
      onclick={addVision}><CirclePlus /></button
    >
  </div>

  <div class="input-group grid-cols-6">
    <div class="ig-cell preset-tonal col-span-3">Passive Perception</div>
    <input
      class="ig-input text-center col-span-3"
      type="number"
      placeholder="10"
    />

    {#each monster.visions as vision}
      <input
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="60"
        bind:value={vision.range}
      />
      <select class="ig-select text-center col-span-4" bind:value={vision.type}>
        {#each Sights as sight}
          <option>{sight}</option>
        {/each}
      </select>
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={removeVision(vision)}><CircleX /></button
      >
    {/each}
  </div>

  <!-- Languages -->
  <hr class="hr" />
  <h2 class="h2">Languages</h2>

  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each Languages as language}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input
          class="checkbox"
          type="checkbox"
          bind:checked={monster.languages[language]}
        />
        <p>{language}</p>
      </label>
    {/each}
  </div>

  <!-- Skills -->
  <hr class="hr" />
  <h2 class="h2">Skills</h2>

  <div class="input-group gap-y-1 px-1 py-1 grid-cols-6">
    {#each Skills as skill, index}
      <div class="ig-cell preset-tonal col-span-1">{skill}</div>
      <input
        class="ig-input text-center col-span-1"
        type="number"
        bind:value={monster.skills[skill]}
      />
      <!-- This is nasty, but draws the horizontal rulers correctly. -->
      {#if index !== 0 && index !== Skills.length - 1 && index % 3 === 2}
        <hr class="hr col-span-6" />
      {/if}
    {/each}
  </div>

  <!-- Traits -->
  <hr class="hr" />
  <div class="flex justify-between">
    <h2 class="h2">Traits</h2>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={addTrait}><CirclePlus /></button
    >
  </div>

  <div class="input-group grid-cols-8">
    {#each monster.traits as trait}
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={trait.name}
        class="ig-input col-span-6"
        type="text"
        placeholder="Martial Advantage"
      />
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={removeTrait(trait)}><CircleX /></button
      >

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-8">Description</div>
      <textarea
        bind:value={trait.description}
        class="ig-input text-area col-span-8"
        rows="4"
        placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated"
      ></textarea>
      <hr class="col-span-8" />
    {/each}
  </div>

  <hr class="hr" />
  <h2 class="h2">Actions</h2>

  <!-- Actions -->
  <div class="input-group grid-cols-8">
    <!-- Regular Actions -->
    <!-- Attack Actions -->
    <!-- Recharge Actions -->
    <!-- Bonus Actions -->
    <!-- Reaction Actions -->
    <!-- Legendary Actions -->
  </div>

  <!-- <button type="button" class="btn" onclick={(e) => $inspect(monster)} -->
  <!--   >Save Monster</button -->
  <!-- > -->

  <div class="h-[100px]"></div>
</form>
