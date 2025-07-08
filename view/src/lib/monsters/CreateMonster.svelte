<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";

  import * as Tabs from "$components/ui/tabs/index";
  import { Button } from "$components/ui/button/index";

  import { Alignments } from "$types/Alignment";
  import { Attributes } from "$types/Attribute";
  import { Conditions } from "$types/Condition";
  import { DamageTypes } from "$types/DamageType";
  import { Languages } from "$types/Language";
  import { MonsterTypes } from "$types/MonsterType";
  import { Movements } from "$types/Movement";
  import { Sights } from "$types/Sight";
  import { Sizes } from "$types/Size";
  import { Skills } from "$types/Skill";
  import { Monster } from "$types/Monster.svelte";
  import { Recharges } from "$types/Recharge";

  import Input from "$components/Input.svelte";
  import Select from "$components/Select.svelte";
  import Checkbox from "$components/Checkbox.svelte";
  import TextArea from "$components/TextArea.svelte";

  let monster = new Monster();

  const sizes = Sizes.map((size) => ({
    value: size,
    label: size,
  }));

  const monsterTypes = MonsterTypes.map((monsterType) => ({
    value: monsterType,
    label: monsterType,
  }));

  const alignments = Alignments.map((alignment) => ({
    value: alignment,
    label: alignment,
  }));

  const sights = Sights.map((sight) => ({ value: sight, label: sight }));

  const movements = Movements.map((movement) => ({
    value: movement,
    label: movement,
  }));

  const damageTypes = DamageTypes.map((damageType) => ({
    value: damageType,
    label: damageType,
  }));
</script>

<Tabs.Root value="basic" class="mx-auto w-[1000px]">
  <div class="flex w-full justify-center">
    <Tabs.List class="flex w-full justify-center">
      <Tabs.Trigger value="basic">Basic Information</Tabs.Trigger>
      <Tabs.Trigger value="defensive">Defensive</Tabs.Trigger>
      <Tabs.Trigger value="traits">Traits</Tabs.Trigger>
      <Tabs.Trigger value="actions">Actions</Tabs.Trigger>
      <Tabs.Trigger value="spellcasting">Spellcasting</Tabs.Trigger>
    </Tabs.List>
  </div>

  <Tabs.Content value="basic" class="mt-5 space-y-5">
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Basic Monster Information
    </h2>
    <div class="grid grid-cols-16 space-y-5 gap-x-2">
      <!-- Name -->
      <Input
        label="Name"
        bind:value={monster.name}
        type="text"
        placeholder="Goblin"
        containerClass="col-span-7"
      />

      <!-- Challenge Rating -->
      <Input
        label="Challenge Rating"
        bind:value={monster.challengeRating}
        type="number"
        placeholder="0.5"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- XP -->
      <Input
        label="XP"
        bind:value={monster.xp}
        type="number"
        placeholder="100"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Proficiency Bonus -->
      <Input
        label="Proficiency Bonus"
        bind:value={monster.proficiencyBonus}
        type="number"
        placeholder="2"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Species -->
      <Input
        label="Species"
        bind:value={monster.species}
        type="text"
        placeholder="Goblinoid"
        containerClass="col-span-4"
      />

      <!-- Type -->
      <Select
        label="Type"
        placeholder="Select a type"
        bind:value={monster.monsterType}
        items={monsterTypes}
        containerClass="col-span-3"
      />

      <!-- Size -->
      <Select
        label="Size"
        placeholder="Select a size"
        bind:value={monster.size}
        items={sizes}
        containerClass="col-span-3"
      />

      <!-- Alignment -->
      <Select
        label="Alignment"
        placeholder="Select an alignment"
        bind:value={monster.alignment}
        items={alignments}
        containerClass="col-span-3"
      />

      <!-- Passive Perception -->
      <Input
        label="Passive Perception"
        bind:value={monster.passivePerception}
        type="number"
        placeholder="13"
        containerClass="col-span-3"
        inputClass="text-center"
      />
    </div>

    <!-- Attributes -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Attributes</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Attributes -->
      {#each Attributes as attribute}
        <Input
          label={attribute}
          placeholder="10"
          bind:value={monster.attributes[attribute]}
          type="number"
          containerClass="col-span-3"
          inputClass="text-center"
        />
      {/each}
    </div>

    <!-- Skills -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Skills</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      {#each Skills as skill}
        <Input
          label={skill}
          bind:value={monster.skills[skill]}
          type="number"
          placeholder=""
          containerClass="col-span-3"
          inputClass="text-center"
        />
      {/each}
    </div>

    <!-- Languages -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Languages</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each Languages as language}
        <Checkbox
          label={language}
          bind:checked={monster.languages[language]}
          columns={1}
        />
      {/each}
    </div>

    <div class="grid grid-cols-2 space-y-5 gap-x-2">
      <!-- Vision -->
      <div class="col-span-1">
        <div class="grid grid-cols-10 space-y-5 gap-x-2">
          <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Vision</h2>
          <!-- Add Vission Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <Button
              variant="ghost"
              size="icon"
              class="text-green-300 hover:text-green-600"
              onclick={(e) => monster.AddVision(e)}
            >
              <CirclePlus />
            </Button>
          </div>

          <!-- Vision List -->
          {#each monster.visions as vision}
            <!-- Range -->
            <Input
              label="Range"
              placeholder="60"
              type="number"
              containerClass="col-span-3"
              inputClass="text-center"
              bind:value={vision.range}
            />

            <!-- Sight Type -->
            <Select
              label="Sight"
              items={sights}
              bind:value={vision.type}
              placeholder="Select a vision type"
              containerClass="col-span-6"
            />

            <!-- Remove Vision Button -->
            <div class="col-span-1 col-start-10 flex justify-center">
              <Button
                variant="ghost"
                size="icon"
                class="mt-5 text-red-300 hover:text-red-600"
                onclick={(_) => monster.RemoveVision(vision)}
              >
                <CircleX />
              </Button>
            </div>
          {/each}
        </div>
      </div>

      <!-- Speed -->
      <div class="col-span-1">
        <div class="grid grid-cols-10 space-y-5 gap-x-2">
          <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Speed</h2>
          <!-- Add Speed Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <Button
              variant="ghost"
              size="icon"
              class="text-green-300 hover:text-green-600"
              onclick={(e) => monster.AddSpeed(e)}
            >
              <CirclePlus />
            </Button>
          </div>

          <!-- Speed List -->
          {#each monster.speeds as speed}
            <!-- Range -->
            <Input
              label="Range"
              placeholder="60"
              type="number"
              containerClass="col-span-3"
              inputClass="text-center"
              bind:value={speed.range}
            />

            <!-- Movement Type -->
            <Select
              label="Movement"
              items={movements}
              bind:value={speed.type}
              placeholder="Select a movement type"
              containerClass="col-span-6"
            />

            <!-- Remove Speed Button -->
            <div class="col-span-1 col-start-10 flex justify-center">
              <Button
                variant="ghost"
                size="icon"
                class="mt-5 text-red-300 hover:text-red-600"
                onclick={(_) => monster.RemoveSpeed(speed)}
              >
                <CircleX />
              </Button>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </Tabs.Content>
  <Tabs.Content value="defensive" class="mt-5">
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Basic Defensive Information
    </h2>
    <div class="grid grid-cols-16 space-y-5 gap-x-2">
      <!-- Hit Points -->
      <Input
        label="Hit Points"
        placeholder="11"
        bind:value={monster.hitPoints}
        type="number"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Rollable Hit Points -->
      <Input
        label="Rollable Hit Points"
        placeholder="2d8 + 6"
        bind:value={monster.rollableHitPoints}
        type="text"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Armor Class -->
      <Input
        label="Armor Class"
        placeholder="18"
        bind:value={monster.armorClass}
        type="number"
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Armor Type -->
      <Input
        label="Armor Type"
        placeholder="chain mail, shield"
        bind:value={monster.armorType}
        type="text"
        containerClass="col-span-7"
      />
    </div>

    <!-- Saving Throws -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Saving Throws</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Saving Throws -->
      {#each Attributes as attribute}
        <Input
          label={attribute}
          bind:value={monster.savingThrows[attribute]}
          type="number"
          placeholder=""
          containerClass="col-span-3"
        />
      {/each}
    </div>

    <!-- Damage Resistances -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Damage Resistances</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Checkbox
          label={damageType}
          bind:checked={monster.damageResistances[damageType]}
          columns={1}
        />
      {/each}
    </div>

    <!-- Damage Immunities -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Damage Immunities</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Checkbox
          label={damageType}
          bind:checked={monster.damageImmunities[damageType]}
          columns={1}
        />
      {/each}
    </div>

    <!-- Condition Immunities -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">
      Condition Immunities
    </h2>
    <div class="grid grid-cols-5 space-y-2 gap-x-2">
      {#each Conditions as condition}
        <Checkbox
          label={condition}
          bind:checked={monster.conditionImmunities[condition]}
          columns={1}
        />
      {/each}
    </div>
  </Tabs.Content>
  <Tabs.Content value="traits" class="mt-5">
    <div class="grid grid-cols-10 space-y-5 gap-x-2">
      <!-- Traits -->
      <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">Traits</h2>
      <!-- Add Trait Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="w-full text-green-300 hover:text-green-600"
          onclick={(e) => monster.AddTrait(e)}
        >
          <CirclePlus />
        </Button>
      </div>

      {#each monster.traits as trait, index}
        <!-- Name -->
        <Input
          label="Name"
          placeholder="Martial Advantage"
          type="text"
          containerClass="col-span-9"
          bind:value={trait.name}
        />

        <!-- Remove Trait Button -->
        <div class="col-span-1 col-start-10 flex justify-center">
          <Button
            variant="ghost"
            size="icon"
            class="mt-5 w-full text-red-300 hover:text-red-600"
            onclick={(_) => monster.RemoveTrait(trait)}
          >
            <CircleX />
          </Button>
        </div>

        <TextArea
          label="Description"
          bind:value={trait.description}
          placeholder="xdd"
          id="trait-{index}"
          columns={10}
        />
      {/each}
    </div>
  </Tabs.Content>
  <Tabs.Content
    value="actions"
    class="mt-5 grid grid-cols-10 space-y-5 gap-x-2"
  >
    <!-- Regular Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Regular Actions
    </h2>
    <!-- Add Regular Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddRegularAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.regularActions as regularAction, index}
      <!-- Name -->
      <Input
        label="Name"
        placeholder="Martial Advantage"
        type="text"
        containerClass="col-span-9"
        bind:value={regularAction.name}
      />

      <!-- Remove Regular Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveRegularAction(regularAction)}
        >
          <CircleX />
        </Button>
      </div>

      <TextArea
        label="Description"
        bind:value={regularAction.description}
        placeholder="xdd"
        id="trait-{index}"
        columns={10}
      />

      {#if index !== monster.regularActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Melee Attack Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Melee Attack Actions
    </h2>
    <!-- Add Melee Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddMeleeAttackAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.meleeAttackActions as meleeAttackAction, index}
      <!-- Name -->
      <Input
        label="Name"
        placeholder="Longsword"
        type="text"
        containerClass="col-span-5"
        bind:value={meleeAttackAction.name}
      />

      <!-- Bonus to Hit -->
      <Input
        label="Bonus to Hit"
        placeholder="5"
        type="number"
        bind:value={meleeAttackAction.hitBonus}
        containerClass="col-span-2"
        inputClass="text-center"
      />

      <!-- Reach -->
      <Input
        label="Reach"
        placeholder="5"
        type="number"
        bind:value={meleeAttackAction.reach}
        containerClass="col-span-2"
        inputClass="text-center"
      />

      <!-- Remove Melee Attack Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveMeleeAttackAction(meleeAttackAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- One-Handed Attack -->
      <Input
        label="One-Handed Attack"
        placeholder="1d8 + 1"
        type="text"
        bind:value={meleeAttackAction.oneHandedAttack}
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Two-Handed Attack -->
      <Input
        label="Two-Handed Attack"
        placeholder="1d10 + 1"
        type="text"
        bind:value={meleeAttackAction.twoHandedAttack}
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <Select
        label="Damage Type"
        bind:value={meleeAttackAction.damageType}
        placeholder="Select a damage type"
        items={damageTypes}
        containerClass="col-span-4"
      />

      {#if index !== monster.meleeAttackActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Ranged Attack Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Ranged Attack Actions
    </h2>
    <!-- Add Ranged Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddRangedAttackAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.rangedAttackActions as rangedAttackAction, index}
      <!-- Name -->
      <Input
        label="Name"
        placeholder="Longsword"
        type="text"
        containerClass="col-span-5"
        bind:value={rangedAttackAction.name}
      />

      <!-- Bonus to Hit -->
      <Input
        label="Bonus to Hit"
        placeholder="5"
        type="number"
        bind:value={rangedAttackAction.hitBonus}
        containerClass="col-span-2"
        inputClass="text-center"
      />

      <!-- Attack -->
      <Input
        label="Attack"
        placeholder="1d8 + 2"
        type="text"
        bind:value={rangedAttackAction.attack}
        containerClass="col-span-2"
        inputClass="text-center"
      />

      <!-- Remove Ranged Attack Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveRangedAttackAction(rangedAttackAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Normal Range -->
      <Input
        label="Normal Range"
        placeholder="150"
        type="number"
        bind:value={rangedAttackAction.normalRange}
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <!-- Long Range -->
      <Input
        label="Long Range"
        placeholder="600"
        type="number"
        bind:value={rangedAttackAction.longRange}
        containerClass="col-span-3"
        inputClass="text-center"
      />

      <Select
        label="Damage Type"
        bind:value={rangedAttackAction.damageType}
        placeholder="Select a damage type"
        items={damageTypes}
        containerClass="col-span-4"
      />

      {#if index !== monster.rangedAttackActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}
  </Tabs.Content>
  <Tabs.Content value="spellcasting" class="mt-5">Spellcasting</Tabs.Content>
</Tabs.Root>

<!-- <form class="space-y-2 py-2"> -->
<!--   <!-- Melee Attack Actions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Melee Attack Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddMeleeAttackAction(event)} -->
<!--       ><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.meleeAttackActions as meleeAttackAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={meleeAttackAction.name} -->
<!--         type="text" -->
<!--         placeholder="Longsword" -->
<!--         labelSize={1} -->
<!--         inputSize={8} -->
<!--       /> -->
<!---->
<!--       <!-- Bonus to Hit -->
<!--       <Input -->
<!--         label="Bonus to Hit" -->
<!--         bind:value={meleeAttackAction.hitBonus} -->
<!--         type="number" -->
<!--         placeholder="3" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Reach -->
<!--       <Input -->
<!--         label="Reach" -->
<!--         bind:value={meleeAttackAction.reach} -->
<!--         type="number" -->
<!--         placeholder="5" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Melee Attack Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveMeleeAttackAction(meleeAttackAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- One-Handed Attack -->
<!--       <Input -->
<!--         label="One-Handed Attack" -->
<!--         bind:value={meleeAttackAction.oneHandedAttack} -->
<!--         type="text" -->
<!--         placeholder="1d8 + 1" -->
<!--         labelSize={3} -->
<!--         inputSize={2} -->
<!--         center={true} -->
<!--       /> -->
<!---->
<!--       <!-- Two-Handed Attack -->
<!--       <Input -->
<!--         label="Two-Handed Attack" -->
<!--         bind:value={meleeAttackAction.twoHandedAttack} -->
<!--         type="text" -->
<!--         placeholder="1d10 + 1" -->
<!--         labelSize={3} -->
<!--         inputSize={2} -->
<!--         center={true} -->
<!--       /> -->
<!---->
<!--       <!-- Damage Type -->
<!--       <SelectInput -->
<!--         title="Damage Type" -->
<!--         bind:value={meleeAttackAction.damageType} -->
<!--         items={DamageTypes} -->
<!--         labelSize={2} -->
<!--         inputSize={4} -->
<!--       /> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Ranged Attack Actions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Ranged Attack Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddRangedAttackAction(event)} -->
<!--       ><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.rangedAttackActions as rangedAttackAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={rangedAttackAction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={11} -->
<!--       /> -->
<!---->
<!--       <!-- Bonus to Hit -->
<!--       <Input -->
<!--         label="Bonus to Hit" -->
<!--         bind:value={rangedAttackAction.hitBonus} -->
<!--         type="number" -->
<!--         placeholder="3" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Ranged Attack Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveRangedAttackAction(rangedAttackAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Normal Range -->
<!--       <Input -->
<!--         label="Normal Range" -->
<!--         bind:value={rangedAttackAction.normalRange} -->
<!--         type="number" -->
<!--         placeholder="80" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Long Range -->
<!--       <Input -->
<!--         label="Long Range" -->
<!--         bind:value={rangedAttackAction.longRange} -->
<!--         type="number" -->
<!--         placeholder="320" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Attack -->
<!--       <Input -->
<!--         label="Attack" -->
<!--         bind:value={rangedAttackAction.attack} -->
<!--         type="text" -->
<!--         placeholder="1d8 + 1" -->
<!--         labelSize={2} -->
<!--         inputSize={2} -->
<!--         center={true} -->
<!--       /> -->
<!---->
<!--       <!-- Damage Type -->
<!--       <SelectInput -->
<!--         title="Damage Type" -->
<!--         bind:value={rangedAttackAction.damageType} -->
<!--         items={DamageTypes} -->
<!--         labelSize={2} -->
<!--         inputSize={4} -->
<!--       /> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Recharge Actions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Recharge Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddRechargeAction(event)} -->
<!--       ><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.rechargeActions as rechargeAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={rechargeAction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={10} -->
<!--       /> -->
<!---->
<!--       <!-- Recharge Dice -->
<!--       <SelectInput -->
<!--         title="Recharge Dice" -->
<!--         bind:value={rechargeAction.rechargeDice} -->
<!--         items={Recharges} -->
<!--         labelSize={2} -->
<!--         inputSize={2} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Recharge Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveRechargeAction(rechargeAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Description -->
<!--       <div class="ig-cell preset-tonal col-span-16 h-8">Description</div> -->
<!--       <hr class="hr col-span-16" /> -->
<!--       <textarea -->
<!--         bind:value={rechargeAction.description} -->
<!--         class="ig-input text-area col-span-16" -->
<!--         rows="4" -->
<!--         placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated" -->
<!--       ></textarea> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Bonus Actions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Bonus Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddBonusAction(event)}><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.bonusActions as bonusAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={bonusAction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={14} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Bonus Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveBonusAction(bonusAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Description -->
<!--       <div class="ig-cell preset-tonal col-span-16 h-8">Description</div> -->
<!--       <hr class="hr col-span-16" /> -->
<!--       <textarea -->
<!--         bind:value={bonusAction.description} -->
<!--         class="ig-input text-area col-span-16" -->
<!--         rows="4" -->
<!--         placeholder="Once per turn, the hobgoblin can deal an extra 7 (2d6) damage to a creature it hits with a weapon attack if that creature is within 5 feet of an ally of the hobgoblin that isn't incapacitated" -->
<!--       ></textarea> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Reactions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Reactions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddReaction(event)}><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.reactions as reaction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={reaction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={14} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Reaction -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveReaction(reaction)}><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Description -->
<!--       <div class="ig-cell preset-tonal col-span-16 h-8">Description</div> -->
<!--       <hr class="hr col-span-16" /> -->
<!--       <textarea -->
<!--         bind:value={reaction.description} -->
<!--         class="ig-input text-area col-span-16" -->
<!--         rows="4" -->
<!--         placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon." -->
<!--       ></textarea> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Legendary Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddLegendaryAction(event)} -->
<!--       ><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#if monster.legendaryActions.length > 0} -->
<!--     <div class="input-group grid-cols-3"> -->
<!--       <Input -->
<!--         label="Available Legendary Actions per Turn" -->
<!--         bind:value={monster.availableLegendaryActionsPerTurn} -->
<!--         type="number" -->
<!--         placeholder="3" -->
<!--         labelSize={2} -->
<!--         inputSize={1} -->
<!--       /> -->
<!--     </div> -->
<!--   {/if} -->
<!--   {#each monster.legendaryActions as legendaryAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={legendaryAction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={12} -->
<!--       /> -->
<!---->
<!--       <!-- Cost -->
<!--       <Input -->
<!--         label="Cost" -->
<!--         bind:value={legendaryAction.cost} -->
<!--         type="number" -->
<!--         placeholder="1" -->
<!--         labelSize={1} -->
<!--         inputSize={1} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Legendary Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveLegendaryAction(legendaryAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Description -->
<!--       <div class="ig-cell preset-tonal col-span-16 h-8">Description</div> -->
<!--       <hr class="hr col-span-16" /> -->
<!--       <textarea -->
<!--         bind:value={legendaryAction.description} -->
<!--         class="ig-input text-area col-span-16" -->
<!--         rows="4" -->
<!--         placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon." -->
<!--       ></textarea> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Lair Actions -->
<!--   <div class="flex justify-between"> -->
<!--     <h6 class="h6">Lair Actions</h6> -->
<!--     <button -->
<!--       type="button" -->
<!--       class="btn text-success-500 border-none" -->
<!--       onclick={(event) => monster.AddLairAction(event)}><CirclePlus /></button -->
<!--     > -->
<!--   </div> -->
<!--   {#each monster.lairActions as lairAction} -->
<!--     <div class="input-group grid-cols-16"> -->
<!--       <!-- Name -->
<!--       <Input -->
<!--         label="Name" -->
<!--         bind:value={lairAction.name} -->
<!--         type="text" -->
<!--         placeholder="Light Crossbow" -->
<!--         labelSize={1} -->
<!--         inputSize={14} -->
<!--       /> -->
<!---->
<!--       <!-- Remove Lair Action -->
<!--       <button -->
<!--         type="button" -->
<!--         class="btn preset-tonal text-error-300 col-span-1" -->
<!--         onclick={(_) => monster.RemoveLairAction(lairAction)} -->
<!--         ><CircleX /></button -->
<!--       > -->
<!---->
<!--       <hr class="hr col-span-16" /> -->
<!---->
<!--       <!-- Description -->
<!--       <div class="ig-cell preset-tonal col-span-16 h-8">Description</div> -->
<!--       <hr class="hr col-span-16" /> -->
<!--       <textarea -->
<!--         bind:value={lairAction.description} -->
<!--         class="ig-input text-area col-span-16" -->
<!--         rows="4" -->
<!--         placeholder="The captain adds 2 to its AC against one melee attack that would hit it. To do so, the captain must see the attacker and be wielding a melee weapon." -->
<!--       ></textarea> -->
<!--     </div> -->
<!--   {/each} -->
<!---->
<!--   <!-- Spellcasting -->
<!--   <hr class="hr" /> -->
<!--   <h2 class="h2">Spellcasting</h2> -->
<!--   <div class="input-group grid-cols-12"> -->
<!--     <!-- Level -->
<!--     <Input -->
<!--       label="Level" -->
<!--       bind:value={monster.spellcastingLevel} -->
<!--       type="number" -->
<!--       placeholder="12" -->
<!--       labelSize={1} -->
<!--       inputSize={1} -->
<!--     /> -->
<!---->
<!--     <!-- Attribute -->
<!--     <SelectInput -->
<!--       title="Attribute" -->
<!--       bind:value={monster.spellcastingAttribute} -->
<!--       items={Attributes} -->
<!--       labelSize={2} -->
<!--       inputSize={3} -->
<!--     /> -->
<!---->
<!--     <!-- DC -->
<!--     <Input -->
<!--       label="DC" -->
<!--       bind:value={monster.spellcastingDC} -->
<!--       type="number" -->
<!--       placeholder="15" -->
<!--       labelSize={1} -->
<!--       inputSize={1} -->
<!--     /> -->
<!---->
<!--     <Input -->
<!--       label="Attack Bonus" -->
<!--       bind:value={monster.spellcastingAttackBonus} -->
<!--       type="number" -->
<!--       placeholder="7" -->
<!--       labelSize={2} -->
<!--       inputSize={1} -->
<!--     /> -->
<!--   </div> -->
<!---->
<!--   <!-- <button type="button" class="btn" onclick={(e) => $inspect(monster)} -->

<!--   <!--   >Save Monster</button -->
<!--   <!-- > -->
<!---->
<!--   <div class="h-[500px]"></div> -->
<!-- </form> -->
