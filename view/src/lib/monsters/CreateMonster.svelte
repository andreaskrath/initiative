<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import Dices from "@lucide/svelte/icons/dices";

  import { Alignments } from "../types/Alignment";
  import { Attributes } from "../types/Attribute";
  import { Conditions } from "../types/Condition";
  import { DamageTypes } from "../types/DamageType";
  import { Languages } from "../types/Language";
  import { MonsterTypes } from "../types/MonsterType";
  import { Sights } from "../types/Sight";
  import { Sizes } from "../types/Size";
  import { Skills } from "../types/Skill";
  import { Monster } from "../types/Monster.svelte";
  import { Recharges } from "../types/Recharge";

  let monster = new Monster();
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

  <!-- Condition Immunities -->
  <h6 class="h6">Condition Immunities</h6>
  <div class="input-group grid-cols-5 border-none outline-none py-2 px-2">
    {#each Conditions as condition}
      <label
        class="flex items-center space-x-2 space-y-2 border-none outline-none"
      >
        <input
          class="checkbox"
          type="checkbox"
          bind:checked={monster.conditionImmunities[condition]}
        />
        <p>{condition}</p>
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
      onclick={(event) => monster.AddVision(event)}><CirclePlus /></button
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
        onclick={(_) => monster.RemoveVision(vision)}><CircleX /></button
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

  <div class="input-group grid-cols-6">
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
      onclick={(event) => monster.AddTrait(event)}><CirclePlus /></button
    >
  </div>
  {#each monster.traits as trait}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={trait.name}
        class="ig-input col-span-14"
        type="text"
        placeholder="Martial Advantage"
      />
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveTrait(trait)}><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={trait.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated"
      ></textarea>
      <hr class="col-span-8" />
    </div>
  {/each}

  <!-- Actions -->
  <hr class="hr" />
  <h2 class="h2">Actions</h2>

  <!-- Regular Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Regular Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddRegularAction(event)}
      ><CirclePlus /></button
    >
  </div>
  {#each monster.regularActions as regularAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={regularAction.name}
        class="ig-input col-span-14"
        type="text"
        placeholder="Martial Advantage"
      />
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveRegularAction(regularAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={regularAction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated"
      ></textarea>
    </div>
  {/each}

  <!-- Melee Attack Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Melee Attack Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddMeleeAttackAction(event)}
      ><CirclePlus /></button
    >
  </div>
  {#each monster.meleeAttackActions as meleeAttackAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={meleeAttackAction.name}
        class="ig-input col-span-8"
        type="text"
        placeholder="Longsword"
      />

      <!-- Bonus to Hit -->
      <div class="ig-cell preset-tonal col-span-2">Bonus to Hit</div>
      <input
        bind:value={meleeAttackAction.hitBonus}
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="3"
      />

      <!-- Reach -->
      <div class="ig-cell preset-tonal col-span-2">Reach</div>
      <input
        bind:value={meleeAttackAction.reach}
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="5"
      />

      <!-- Remove Melee Attack Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveMeleeAttackAction(meleeAttackAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- One-Handed Attack -->
      <div class="ig-cell preset-tonal col-span-3">One-Handed Attack</div>
      <input
        bind:value={meleeAttackAction.oneHandedAttack}
        class="ig-input text-center col-span-2"
        type="number"
        placeholder="1d8 + 1"
      />

      <!-- Two-Handed Attack -->
      <div class="ig-cell preset-tonal col-span-3">Two-Handed Attack</div>
      <input
        bind:value={meleeAttackAction.twoHandedAttack}
        class="ig-input text-center col-span-2"
        type="number"
        placeholder="1d10 + 1"
      />

      <!-- Damage Type -->
      <div class="ig-cell preset-tonal col-span-2">Damage Type</div>
      <select
        bind:value={meleeAttackAction.damageType}
        class="ig-select col-span-4"
      >
        {#each DamageTypes as damageType}
          <option>
            {damageType}
          </option>
        {/each}
      </select>
    </div>
  {/each}

  <!-- Ranged Attack Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Ranged Attack Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddRangedAttackAction(event)}
      ><CirclePlus /></button
    >
  </div>
  {#each monster.rangedAttackActions as rangedAttackAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={rangedAttackAction.name}
        class="ig-input col-span-11"
        type="text"
        placeholder="Longsword"
      />

      <!-- Bonus to Hit -->
      <div class="ig-cell preset-tonal col-span-2">Bonus to Hit</div>
      <input
        bind:value={rangedAttackAction.hitBonus}
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="3"
      />

      <!-- Remove Melee Attack Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveRangedAttackAction(rangedAttackAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Normal Range -->
      <div class="ig-cell preset-tonal col-span-2">Normal Range</div>
      <input
        bind:value={rangedAttackAction.normalRange}
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="5"
      />

      <div class="ig-cell preset-tonal col-span-2">Long Range</div>
      <input
        bind:value={rangedAttackAction.longRange}
        class="ig-input text-center col-span-1"
        type="number"
        placeholder="5"
      />

      <!-- Attack -->
      <div class="ig-cell preset-tonal col-span-2">Attack</div>
      <input
        bind:value={rangedAttackAction.attack}
        class="ig-input text-center col-span-2"
        type="number"
        placeholder="1d8 + 1"
      />

      <!-- Damage Type -->
      <div class="ig-cell preset-tonal col-span-2">Damage Type</div>
      <select
        bind:value={rangedAttackAction.damageType}
        class="ig-select col-span-4"
      >
        {#each DamageTypes as damageType}
          <option>
            {damageType}
          </option>
        {/each}
      </select>
    </div>
  {/each}

  <!-- Recharge Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Recharge Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddRechargeAction(event)}
      ><CirclePlus /></button
    >
  </div>
  {#each monster.rechargeActions as rechargeAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={rechargeAction.name}
        class="ig-input col-span-10"
        type="text"
        placeholder="Longsword"
      />

      <!-- Recharge dice -->
      <div class="ig-cell preset-tonal col-span-2">Recharge Dice</div>
      <select
        bind:value={rechargeAction.rechargeDice}
        class="ig-select text-center col-span-2"
      >
        {#each Recharges as recharge}
          <option>
            {recharge}
          </option>
        {/each}
      </select>

      <!-- Remove Recharge Attack Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveRechargeAction(rechargeAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={rechargeAction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated"
      ></textarea>
    </div>
  {/each}

  <!-- Bonus Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Bonus Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddBonusAction(event)}><CirclePlus /></button
    >
  </div>
  {#each monster.bonusActions as bonusAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={bonusAction.name}
        class="ig-input col-span-14"
        type="text"
        placeholder="Longsword"
      />

      <!-- Remove Bonus Attack Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveBonusAction(bonusAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={bonusAction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated"
      ></textarea>
    </div>
  {/each}

  <!-- Reactions -->
  <div class="flex justify-between">
    <h6 class="h6">Reactions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddReaction(event)}><CirclePlus /></button
    >
  </div>
  {#each monster.reactions as reaction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <div class="ig-cell preset-tonal col-span-1">Name</div>
      <input
        bind:value={reaction.name}
        class="ig-input col-span-14"
        type="text"
        placeholder="Parry"
      />

      <!-- Remove Reaction -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveReaction(reaction)}><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={reaction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon."
      ></textarea>
    </div>
  {/each}

  <!-- Reaction Actions -->
  <!-- Legendary Actions -->

  <!-- <button type="button" class="btn" onclick={(e) => $inspect(monster)} -->
  <!--   >Save Monster</button -->
  <!-- > -->

  <div class="h-[500px]"></div>
</form>
