<script lang="ts">
  import Dices from "@lucide/svelte/icons/dices";
  import { DamageType, DamageTypes } from "../types/DamageType";
  import { Size, Sizes } from "../types/Size";
  import { MonsterType, MonsterTypes } from "../types/MonsterType";
  import { Alignment, Alignments } from "../types/Alignment";
  import { Sight, Sights } from "../types/Sight";
  import { Language, Languages } from "../types/Language";
  import { Skill, Skills } from "../types/Skill";

  let visions: { type: Sight; range: number }[] = [];

  function addVision() {
    visions = [...visions, { type: Sight.Darkvision, range: 0 }];
  }

  // Fix this shit cause its wonky as hell when deleting in random orders.
  function removeVision(index: number) {
    visions = visions.filter((_, i) => i !== index);

    return function (event: MouseEvent) {
      event.preventDefault();
    };
  }

  // Leave for now - just an example of how to pre-select default in select inputs.
  let monsterSize = Size.Medium;
</script>

<div class="py-2"></div>
<form class="w-[1200px] mx-auto space-y-8 px-2">
  <h2 class="h2">Basic Information</h2>

  <!-- Name, Challenge, XP & Proficiency-->
  <div class="input-group grid-cols-16">
    <!-- Name -->
    <div class="ig-cell preset-tonal col-span-1">Name</div>
    <input class="ig-input col-span-6" type="text" placeholder="Goblin" />

    <!-- Challenge Rating -->
    <div class="ig-cell preset-tonal col-span-2">Challenge</div>
    <input
      class="ig-input col-span-1 text-center no-spinner"
      type="number"
      placeholder="0.5"
    />

    <!-- Experience Points -->
    <div class="ig-cell preset-tonal col-span-1">XP</div>
    <input
      class="ig-input col-span-2 text-center"
      type="number"
      placeholder="100"
    />

    <!-- Proficiency Bonus -->
    <div class="ig-cell preset-tonal col-span-2">Proficiency</div>
    <input
      class="ig-input col-span-1 text-center"
      type="number"
      placeholder="2"
    />
  </div>

  <!-- Size, Type, Species & Alignment -->
  <div class="input-group grid-cols-32">
    <!-- Size -->
    <div class="ig-cell preset-tonal col-span-2">Size</div>
    <select class="ig-select col-span-5 text-center" bind:value={monsterSize}>
      {#each Sizes as size}
        <option>{size}</option>
      {/each}
    </select>

    <!-- Type -->
    <div class="ig-cell preset-tonal col-span-2">Type</div>
    <select class="ig-select col-span-6 text-center">
      {#each MonsterTypes as monsterType}
        <option>{monsterType}</option>
      {/each}
    </select>

    <!-- Species -->
    <div class="ig-cell preset-tonal col-span-2">Species</div>
    <input class="ig-input col-span-6" type="text" placeholder="Goblinoid" />

    <!-- Alignment -->
    <div class="ig-cell preset-tonal col-span-3">Alignment</div>
    <select class="ig-select col-span-6 text-center">
      {#each Alignments as alignment}
        <option>{alignment}</option>
      {/each}
    </select>
  </div>

  <hr class="hr" />
  <h2 class="h2">Attributes</h2>

  <!-- Attributes -->
  <div class="input-group grid-cols-18">
    <!-- Strength -->
    <div class="ig-cell preset-tonal col-span-2">Strength</div>
    <input class="ig-input text-center" type="number" placeholder="13" />

    <!-- Dexterity -->
    <div class="ig-cell preset-tonal col-span-2">Dexterity</div>
    <input class="ig-input text-center" type="number" placeholder="12" />

    <!-- Constitution -->
    <div class="ig-cell preset-tonal col-span-2">Constitution</div>
    <input class="ig-input text-center" type="number" placeholder="12" />

    <!-- Intelligence -->
    <div class="ig-cell preset-tonal col-span-2">Intelligence</div>
    <input class="ig-input text-center" type="number" placeholder="10" />

    <!-- Wisdom -->
    <div class="ig-cell preset-tonal col-span-2">Wisdom</div>
    <input class="ig-input text-center" type="number" placeholder="10" />

    <!-- Charisma -->
    <div class="ig-cell preset-tonal col-span-2">Charisma</div>
    <input class="ig-input text-center" type="number" placeholder="9" />
  </div>

  <hr class="hr" />
  <h2 class="h2">Defensive</h2>

  <!-- Hit Points, Rollable HP, Armor Class & Armor Type -->
  <div class="input-group grid-cols-16">
    <!-- Hit Points -->
    <div class="ig-cell preset-tonal col-span-2">Hit Points</div>
    <input
      class="ig-input col-span-1 text-center"
      type="number"
      placeholder="11"
    />

    <!-- Rollable Hit Points -->
    <div class="ig-cell preset-tonal col-span-1" title="Rollable Hit Points">
      <Dices />
    </div>
    <input
      class="ig-input text-center col-span-2"
      type="text"
      placeholder="2d8 +2"
    />

    <!-- Armor Class -->
    <div class="ig-cell preset-tonal col-span-2">Armor Class</div>
    <input
      class="ig-input text-center col-span-1"
      type="number"
      placeholder="18"
    />

    <!-- Armor Type -->
    <div class="ig-cell preset-tonal col-span-2">Armor Type</div>
    <input
      class="ig-input col-span-5"
      type="text"
      placeholder="chain mail, shield"
    />
  </div>

  <h6 class="h6">Saving Throws</h6>
  <div class="input-group grid-cols-18">
    <!-- Strength -->
    <div class="ig-cell preset-tonal col-span-2">Strength</div>
    <input class="ig-input text-center" type="number" placeholder="3" />

    <!-- Dexterity -->
    <div class="ig-cell preset-tonal col-span-2">Dexterity</div>
    <input class="ig-input text-center" type="number" placeholder="2" />

    <!-- Constitution -->
    <div class="ig-cell preset-tonal col-span-2">Constitution</div>
    <input class="ig-input text-center" type="number" placeholder="" />

    <!-- Intelligence -->
    <div class="ig-cell preset-tonal col-span-2">Intelligence</div>
    <input class="ig-input text-center" type="number" placeholder="" />

    <!-- Wisdom -->
    <div class="ig-cell preset-tonal col-span-2">Wisdom</div>
    <input class="ig-input text-center" type="number" placeholder="" />

    <!-- Charisma -->
    <div class="ig-cell preset-tonal col-span-2">Charisma</div>
    <input class="ig-input text-center" type="number" placeholder="" />
  </div>

  <h6 class="h6">Damage Resistances</h6>

  <!-- Damage Resistances -->
  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each DamageTypes as damageType}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input class="checkbox" type="checkbox" />
        <p>{damageType}</p>
      </label>
    {/each}
  </div>

  <h6 class="h6">Damage Immunities</h6>

  <!-- Damage Immunities -->
  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each DamageTypes as damageType}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input class="checkbox" type="checkbox" />
        <p>{damageType}</p>
      </label>
    {/each}
  </div>

  <hr class="hr" />
  <h2 class="h2">Senses</h2>

  <!-- Vision & Perception -->
  <div class="input-group grid-cols-3">
    <div class="ig-cell preset-tonal col-span-1">Passive Perception</div>
    <input
      class="ig-input text-center col-span-1"
      type="number"
      placeholder="10"
    />
    <Button classes="col-span-1" title="Add New Vision" onClick={addVision} />

    {#each visions as vision, index}
      <input
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="60"
        bind:value={vision.range}
      />
      <select class="ig-select col-span-1" bind:value={vision.type}>
        {#each Sights as sight}
          <option>{sight}</option>
        {/each}
      </select>
      <button
        type="button"
        class="btn preset-filled-secondary col-span-1"
        onclick={removeVision(index)}>Remove Vision</button
      >
    {/each}
  </div>

  <hr class="hr" />
  <h2 class="h2">Languages</h2>

  <!-- Languages -->
  <div class="input-group grid-cols-4 border-none outline-none py-2 px-2">
    {#each Languages as language}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input class="checkbox" type="checkbox" />
        <p>{language}</p>
      </label>
    {/each}
  </div>

  <h2 class="h2">Skills</h2>

  <!-- Skills -->
  <div class="input-group gap-y-1 px-1 py-1 grid-cols-6">
    {#each Skills as skill}
      <div class="ig-cell preset-tonal col-span-1">{skill}</div>
      <input class="ig-input text-center col-span-1" type="number" />
    {/each}
  </div>

  <div class="h-[100px]"></div>
</form>
