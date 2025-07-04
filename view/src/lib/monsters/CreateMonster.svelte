<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";

  import { Alignments } from "$types/Alignment";
  import { Attributes } from "$types/Attribute";
  import { Conditions } from "$types/Condition";
  import { DamageTypes } from "$types/DamageType";
  import { Languages } from "$types/Language";
  import { MonsterTypes } from "$types/MonsterType";
  import { Sights } from "$types/Sight";
  import { Sizes } from "$types/Size";
  import { Skills } from "$types/Skill";
  import { Monster } from "$types/Monster.svelte";
  import { Recharges } from "$types/Recharge";

  import CheckboxGroup from "$components/CheckboxGroup.svelte";
  import Input from "$components/Input.svelte";
  import SelectInput from "$components/SelectInput.svelte";

  let monster = new Monster();
</script>

<form class="space-y-2 py-2">
  <!-- Basic Information -->
  <h2 class="h2">Basic Information</h2>
  <div class="input-group grid-cols-32">
    <!-- Name -->
    <Input
      label="Name"
      bind:value={monster.name}
      type="text"
      placeholder="Goblin"
      labelSize={2}
      inputSize={16}
    />

    <!-- Challenge Rating -->
    <Input
      label="Challenge"
      bind:value={monster.challengeRating}
      type="number"
      placeholder="0.5"
      labelSize={3}
      inputSize={2}
    />

    <!-- Experience Points -->
    <Input
      label="XP"
      bind:value={monster.xp}
      type="number"
      placeholder="100"
      labelSize={1}
      inputSize={3}
    />

    <!-- Proficiency Bonus -->
    <Input
      label="Proficiency"
      bind:value={monster.proficiencyBonus}
      type="number"
      placeholder="2"
      labelSize={3}
      inputSize={2}
    />

    <!-- New Row -->
    <hr class="hr col-span-32" />

    <!-- Size -->
    <SelectInput
      title="Size"
      bind:value={monster.size}
      items={Sizes}
      labelSize={2}
      inputSize={5}
    />

    <!-- Monster Type -->
    <SelectInput
      title="Type"
      bind:value={monster.monsterType}
      items={MonsterTypes}
      labelSize={2}
      inputSize={6}
    />

    <!-- Species -->
    <Input
      label="Species"
      bind:value={monster.species}
      type="text"
      placeholder="Goblinoid"
      labelSize={2}
      inputSize={6}
    />

    <!-- Alignment -->
    <SelectInput
      title="Alignment"
      bind:value={monster.alignment}
      items={Alignments}
      labelSize={3}
      inputSize={6}
    />
  </div>

  <!-- Attributes -->
  <hr class="hr" />
  <h2 class="h2">Attributes</h2>

  <div class="input-group grid-cols-18">
    {#each Attributes as attribute}
      <Input
        label={attribute}
        bind:value={monster.attributes[attribute]}
        type="number"
        placeholder=""
        labelSize={2}
        inputSize={1}
      />
    {/each}
  </div>

  <!-- Defensive -->
  <hr class="hr" />
  <h2 class="h2">Defensive</h2>

  <div class="input-group grid-cols-16">
    <!-- Hit Points -->
    <Input
      label="Hit Points"
      bind:value={monster.hitPoints}
      type="number"
      placeholder="11"
      labelSize={2}
      inputSize={1}
    />

    <!-- Rollable Hit Points -->
    <Input
      label="Rollable"
      bind:value={monster.rollableHitPoints}
      type="text"
      placeholder="2d8 + 2"
      labelSize={2}
      inputSize={2}
      center={true}
    />

    <!-- Armor Class -->
    <Input
      label="Armor Class"
      bind:value={monster.armorClass}
      type="number"
      placeholder="18"
      labelSize={2}
      inputSize={1}
    />

    <!-- Armor Type -->
    <Input
      label="Armor Type"
      bind:value={monster.armorType}
      type="text"
      placeholder="chain mail, shield"
      labelSize={2}
      inputSize={4}
    />
  </div>

  <!-- Saving Throws -->
  <h6 class="h6">Saving Throws</h6>
  <div class="input-group grid-cols-18">
    {#each Attributes as attribute}
      <Input
        label={attribute}
        bind:value={monster.savingThrows[attribute]}
        type="number"
        placeholder=""
        labelSize={2}
        inputSize={1}
      />
    {/each}
  </div>

  <!-- Damage Resistances -->
  <h6 class="h6">Damage Resistances</h6>
  <CheckboxGroup
    items={DamageTypes}
    bind:checkedItems={monster.damageResistances}
    columns={4}
  />

  <!-- Damage Immunities -->
  <h6 class="h6">Damage Immunities</h6>
  <CheckboxGroup
    items={DamageTypes}
    bind:checkedItems={monster.damageImmunities}
    columns={4}
  />

  <!-- Condition Immunities -->
  <h6 class="h6">Condition Immunities</h6>
  <CheckboxGroup
    items={Conditions}
    bind:checkedItems={monster.conditionImmunities}
    columns={5}
  />

  <!-- Senses -->
  <hr class="hr" />
  <div class="flex justify-between">
    <h2 class="h2">Senses</h2>
    <button
      type="button"
      class="btn border-none text-success-300"
      onclick={(event) => monster.AddVision(event)}><CirclePlus /></button
    >
  </div>

  <div class="input-group grid-cols-2">
    <Input
      label="Passive Perception"
      bind:value={monster.passivePerception}
      type="number"
      placeholder="10"
      labelSize={1}
      inputSize={1}
    />
  </div>

  {#each monster.visions as vision}
    <div class="input-group grid-cols-16">
      <!-- Sight Type -->
      <SelectInput
        title="Sight"
        bind:value={vision.type}
        items={Sights}
        labelSize={2}
        inputSize={10}
      />

      <!-- Range -->
      <Input
        label="Range"
        bind:value={vision.range}
        type="number"
        placeholder="60"
        labelSize={2}
        inputSize={1}
      />

      <!-- Remove Sight -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveVision(vision)}><CircleX /></button
      >
    </div>
  {/each}

  <!-- Languages -->
  <hr class="hr" />
  <h2 class="h2">Languages</h2>
  <CheckboxGroup
    items={Languages}
    bind:checkedItems={monster.languages}
    columns={4}
  />

  <!-- Skills -->
  <hr class="hr" />
  <h2 class="h2">Skills</h2>

  <div class="input-group grid-cols-6">
    {#each Skills as skill, index}
      <Input
        label={skill}
        bind:value={monster.skills[skill]}
        type="text"
        placeholder=""
        labelSize={1}
        inputSize={1}
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
      <Input
        label="Name"
        bind:value={trait.name}
        type="text"
        placeholder="Martial Advantage"
        labelSize={1}
        inputSize={14}
      />

      <!-- Remove Trait Button -->
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
      <Input
        label="Name"
        bind:value={regularAction.name}
        type="text"
        placeholder="Martial Advantage"
        labelSize={1}
        inputSize={14}
      />

      <!-- Remove Regular Action Button -->
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
      <Input
        label="Name"
        bind:value={meleeAttackAction.name}
        type="text"
        placeholder="Longsword"
        labelSize={1}
        inputSize={8}
      />

      <!-- Bonus to Hit -->
      <Input
        label="Bonus to Hit"
        bind:value={meleeAttackAction.hitBonus}
        type="number"
        placeholder="3"
        labelSize={2}
        inputSize={1}
      />

      <!-- Reach -->
      <Input
        label="Reach"
        bind:value={meleeAttackAction.reach}
        type="number"
        placeholder="5"
        labelSize={2}
        inputSize={1}
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
      <Input
        label="One-Handed Attack"
        bind:value={meleeAttackAction.oneHandedAttack}
        type="text"
        placeholder="1d8 + 1"
        labelSize={3}
        inputSize={2}
        center={true}
      />

      <!-- Two-Handed Attack -->
      <Input
        label="Two-Handed Attack"
        bind:value={meleeAttackAction.twoHandedAttack}
        type="text"
        placeholder="1d10 + 1"
        labelSize={3}
        inputSize={2}
        center={true}
      />

      <!-- Damage Type -->
      <SelectInput
        title="Damage Type"
        bind:value={meleeAttackAction.damageType}
        items={DamageTypes}
        labelSize={2}
        inputSize={4}
      />
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
      <Input
        label="Name"
        bind:value={rangedAttackAction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={11}
      />

      <!-- Bonus to Hit -->
      <Input
        label="Bonus to Hit"
        bind:value={rangedAttackAction.hitBonus}
        type="number"
        placeholder="3"
        labelSize={2}
        inputSize={1}
      />

      <!-- Remove Ranged Attack Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveRangedAttackAction(rangedAttackAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Normal Range -->
      <Input
        label="Normal Range"
        bind:value={rangedAttackAction.normalRange}
        type="number"
        placeholder="80"
        labelSize={2}
        inputSize={1}
      />

      <!-- Long Range -->
      <Input
        label="Long Range"
        bind:value={rangedAttackAction.longRange}
        type="number"
        placeholder="320"
        labelSize={2}
        inputSize={1}
      />

      <!-- Attack -->
      <Input
        label="Attack"
        bind:value={rangedAttackAction.attack}
        type="text"
        placeholder="1d8 + 1"
        labelSize={2}
        inputSize={2}
        center={true}
      />

      <!-- Damage Type -->
      <SelectInput
        title="Damage Type"
        bind:value={rangedAttackAction.damageType}
        items={DamageTypes}
        labelSize={2}
        inputSize={4}
      />
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
      <Input
        label="Name"
        bind:value={rechargeAction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={10}
      />

      <!-- Recharge Dice -->
      <SelectInput
        title="Recharge Dice"
        bind:value={rechargeAction.rechargeDice}
        items={Recharges}
        labelSize={2}
        inputSize={2}
      />

      <!-- Remove Recharge Action -->
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
      <Input
        label="Name"
        bind:value={bonusAction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={14}
      />

      <!-- Remove Bonus Action -->
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
      <Input
        label="Name"
        bind:value={reaction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={14}
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

  <div class="flex justify-between">
    <h6 class="h6">Legendary Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddLegendaryAction(event)}
      ><CirclePlus /></button
    >
  </div>
  {#if monster.legendaryActions.length > 0}
    <div class="input-group grid-cols-3">
      <Input
        label="Available Legendary Actions per Turn"
        bind:value={monster.availableLegendaryActionsPerTurn}
        type="number"
        placeholder="3"
        labelSize={2}
        inputSize={1}
      />
    </div>
  {/if}
  {#each monster.legendaryActions as legendaryAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <Input
        label="Name"
        bind:value={legendaryAction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={12}
      />

      <!-- Cost -->
      <Input
        label="Cost"
        bind:value={legendaryAction.cost}
        type="number"
        placeholder="1"
        labelSize={1}
        inputSize={1}
      />

      <!-- Remove Legendary Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveLegendaryAction(legendaryAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={legendaryAction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon."
      ></textarea>
    </div>
  {/each}

  <!-- Lair Actions -->
  <div class="flex justify-between">
    <h6 class="h6">Lair Actions</h6>
    <button
      type="button"
      class="btn border-none text-success-500"
      onclick={(event) => monster.AddLairAction(event)}><CirclePlus /></button
    >
  </div>
  {#each monster.lairActions as lairAction}
    <div class="input-group grid-cols-16">
      <!-- Name -->
      <Input
        label="Name"
        bind:value={lairAction.name}
        type="text"
        placeholder="Light Crossbow"
        labelSize={1}
        inputSize={14}
      />

      <!-- Remove Lair Action -->
      <button
        type="button"
        class="btn preset-tonal text-error-300 col-span-1"
        onclick={(_) => monster.RemoveLairAction(lairAction)}
        ><CircleX /></button
      >

      <hr class="hr col-span-16" />

      <!-- Description -->
      <div class="ig-cell preset-tonal h-8 col-span-16">Description</div>
      <hr class="hr col-span-16" />
      <textarea
        bind:value={lairAction.description}
        class="ig-input text-area col-span-16"
        rows="4"
        placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon."
      ></textarea>
    </div>
  {/each}

  <!-- Spellcasting -->
  <hr class="hr" />
  <h2 class="h2">Spellcasting</h2>
  <div class="input-group grid-cols-12">
    <!-- Level -->
    <Input
      label="Level"
      bind:value={monster.spellcastingLevel}
      type="number"
      placeholder="12"
      labelSize={1}
      inputSize={1}
    />

    <!-- Attribute -->
    <SelectInput
      title="Attribute"
      bind:value={monster.spellcastingAttribute}
      items={Attributes}
      labelSize={2}
      inputSize={3}
    />

    <!-- DC -->
    <Input
      label="DC"
      bind:value={monster.spellcastingDC}
      type="number"
      placeholder="15"
      labelSize={1}
      inputSize={1}
    />

    <Input
      label="Attack Bonus"
      bind:value={monster.spellcastingAttackBonus}
      type="number"
      placeholder="7"
      labelSize={2}
      inputSize={1}
    />
  </div>

  <!-- <button type="button" class="btn" onclick={(e) => $inspect(monster)} -->
  <!--   >Save Monster</button -->
  <!-- > -->

  <div class="h-[500px]"></div>
</form>
