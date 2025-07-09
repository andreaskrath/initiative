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
  import Toggle from "$components/Toggle.svelte";
  import TextArea from "$components/TextArea.svelte";
  import Container from "$components/Container.svelte";
  import Label from "$components/Label.svelte";

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

  const recharges = Recharges.map((recharge) => ({
    value: recharge,
    label: recharge,
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
      <Container class="col-span-7">
        <Label>Name</Label>
        <Input bind:value={monster.name} type="text" placeholder="Goblin" />
      </Container>

      <!-- Challenge Rating -->
      <Container class="col-span-3">
        <Label>Challenge Rating</Label>
        <Input
          bind:value={monster.challengeRating}
          type="number"
          placeholder="0.5"
          class="text-center"
        />
      </Container>

      <!-- XP -->
      <Container class="col-span-3">
        <Label>Experience Points</Label>
        <Input
          bind:value={monster.xp}
          type="number"
          placeholder="100"
          class="text-center"
        />
      </Container>

      <!-- Proficiency Bonus -->
      <Container class="col-span-3">
        <Label>Proficiency Bonus</Label>
        <Input
          bind:value={monster.proficiencyBonus}
          type="number"
          placeholder="2"
          class="text-center"
        />
      </Container>

      <!-- Species -->
      <Container class="col-span-4">
        <Label>Species</Label>
        <Input
          bind:value={monster.species}
          type="text"
          placeholder="Goblinoid"
        />
      </Container>

      <!-- Type -->
      <Container class="col-span-3">
        <Label>Type</Label>
        <Select
          bind:value={monster.monsterType}
          placeholder="Select a type"
          items={monsterTypes}
        />
      </Container>

      <!-- Size -->
      <Container class="col-span-3">
        <Label>Size</Label>
        <Select
          bind:value={monster.size}
          placeholder="Select a size"
          items={sizes}
        />
      </Container>

      <!-- Alignment -->
      <Container class="col-span-3">
        <Label>Alignment</Label>
        <Select
          bind:value={monster.alignment}
          placeholder="Select an alignment"
          items={alignments}
        />
      </Container>

      <!-- Passive Perception -->
      <Container class="col-span-3">
        <Label>Passive Perception</Label>
        <Input
          bind:value={monster.passivePerception}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>
    </div>

    <!-- Attributes -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Attributes</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Attributes -->
      {#each Attributes as attribute}
        <Container class="col-span-3">
          <Label>{attribute}</Label>
          <Input
            bind:value={monster.attributes[attribute]}
            type="number"
            placeholder="13"
            class="text-center"
          />
        </Container>
      {/each}
    </div>

    <!-- Skills -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Skills</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      {#each Skills as skill}
        <Container class="col-span-3">
          <Label>{skill}</Label>
          <Input
            bind:value={monster.skills[skill]}
            type="number"
            placeholder=""
            class="text-center"
          />
        </Container>
      {/each}
    </div>

    <!-- Languages -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Languages</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each Languages as language}
        <Container class="col-span-1">
          <Toggle bind:checked={monster.languages[language]}>
            {language}
          </Toggle>
        </Container>
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
            <Container class="col-span-3">
              <Label>Range</Label>
              <Input
                bind:value={vision.range}
                type="number"
                placeholder="60"
                class="text-center"
              />
            </Container>

            <!-- Sight Type -->
            <Container class="col-span-6">
              <Label>Sight</Label>
              <Select
                bind:value={vision.type}
                placeholder="Select a vision type"
                items={sights}
              />
            </Container>

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
            <!-- Distance -->
            <Container class="col-span-3">
              <Label>Distance</Label>
              <Input
                bind:value={speed.distance}
                type="number"
                placeholder="60"
                class="text-center"
              />
            </Container>

            <!-- Movement Type -->
            <Container class="col-span-6">
              <Label>Movement</Label>
              <Select
                bind:value={speed.type}
                placeholder="Select a movement type"
                items={movements}
              />
            </Container>

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
      <Container class="col-span-3">
        <Label>Hit Points</Label>
        <Input
          bind:value={monster.hitPoints}
          type="number"
          placeholder="11"
          class="text-center"
        />
      </Container>

      <!-- Rollable Hit Points -->
      <Container class="col-span-3">
        <Label>Rollable Hit Points</Label>
        <Input
          bind:value={monster.rollableHitPoints}
          type="text"
          placeholder="2d8 + 6"
          class="text-center"
        />
      </Container>

      <!-- Armor Class -->
      <Container class="col-span-3">
        <Label>Armor Class</Label>
        <Input
          bind:value={monster.armorClass}
          type="number"
          placeholder="18"
          class="text-center"
        />
      </Container>

      <!-- Armor Type -->
      <Container class="col-span-7">
        <Label>Armor Type</Label>
        <Input
          bind:value={monster.armorType}
          type="text"
          placeholder="chain mail, shield"
        />
      </Container>
    </div>

    <!-- Saving Throws -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Saving Throws</h2>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Saving Throws -->
      {#each Attributes as attribute}
        <Container class="col-span-3">
          <Label>{attribute}</Label>
          <Input
            bind:value={monster.savingThrows[attribute]}
            type="number"
            placeholder=""
            class="text-center"
          />
        </Container>
      {/each}
    </div>

    <!-- Damage Resistances -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Damage Resistances</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={monster.damageResistances[damageType]}>
            {damageType}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Damage Immunities -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">Damage Immunities</h2>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={monster.damageImmunities[damageType]}>
            {damageType}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Condition Immunities -->
    <h2 class="text-muted-foreground mb-2 ml-1 text-xl">
      Condition Immunities
    </h2>
    <div class="grid grid-cols-5 space-y-2 gap-x-2">
      {#each Conditions as condition}
        <Container class="col-span-1">
          <Toggle bind:checked={monster.conditionImmunities[condition]}>
            {condition}
          </Toggle>
        </Container>
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
        <Container class="col-span-9">
          <Label>Name</Label>
          <Input
            bind:value={trait.name}
            type="text"
            placeholder="Martial Advantage"
          />
        </Container>

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

        <!-- Description -->
        <Container class="col-span-10">
          <Label>Description</Label>
          <TextArea
            bind:value={trait.description}
            placeholder="Write a description for the trait.."
          />
        </Container>

        {#if index !== monster.traits.length - 1}
          <hr class="col-span-10" />
        {/if}
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
      <Container class="col-span-9">
        <Label>Name</Label>
        <Input
          bind:value={regularAction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

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

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={regularAction.description}
          placeholder="Write a description for the regular action.."
        />
      </Container>

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
      <Container class="col-span-5">
        <Label>Name</Label>
        <Input
          bind:value={meleeAttackAction.name}
          type="text"
          placeholder="Longsword"
        />
      </Container>

      <!-- Bonus to Hit -->
      <Container class="col-span-2">
        <Label>Bonus to Hit</Label>
        <Input
          bind:value={meleeAttackAction.hitBonus}
          type="number"
          placeholder="5"
          class="text-center"
        />
      </Container>

      <!-- Reach -->
      <Container class="col-span-2">
        <Label>Reach</Label>
        <Input
          bind:value={meleeAttackAction.reach}
          type="number"
          placeholder="5"
          class="text-center"
        />
      </Container>

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
      <Container class="col-span-3">
        <Label>One-Handed Attack</Label>
        <Input
          bind:value={meleeAttackAction.oneHandedAttack}
          type="text"
          placeholder="1d8 + 1"
          class="text-center"
        />
      </Container>

      <!-- Two-Handed Attack -->
      <Container class="col-span-3">
        <Label>Two-Handed Attack</Label>
        <Input
          bind:value={meleeAttackAction.twoHandedAttack}
          type="text"
          placeholder="1d10 + 1"
          class="text-center"
        />
      </Container>

      <!-- Damage Type -->
      <Container class="col-span-4">
        <Label>Damage Type</Label>
        <Select
          bind:value={meleeAttackAction.damageType}
          placeholder="Select a damage type"
          items={damageTypes}
        />
      </Container>

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
      <Container class="col-span-5">
        <Label>Name</Label>
        <Input
          bind:value={rangedAttackAction.name}
          type="text"
          placeholder="Longbow"
        />
      </Container>

      <!-- Bonus to Hit -->
      <Container class="col-span-2">
        <Label>Bonus to Hit</Label>
        <Input
          bind:value={rangedAttackAction.hitBonus}
          type="number"
          placeholder="5"
          class="text-center"
        />
      </Container>

      <!-- Attack -->
      <Container class="col-span-2">
        <Label>Attack</Label>
        <Input
          bind:value={rangedAttackAction.attack}
          type="text"
          placeholder="1d8 + 2"
          class="text-center"
        />
      </Container>

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
      <Container class="col-span-3">
        <Label>Normal Range</Label>
        <Input
          bind:value={rangedAttackAction.normalRange}
          type="number"
          placeholder="150"
          class="text-center"
        />
      </Container>

      <!-- Long Range -->
      <Container class="col-span-3">
        <Label>Long Range</Label>
        <Input
          bind:value={rangedAttackAction.longRange}
          type="number"
          placeholder="600"
          class="text-center"
        />
      </Container>

      <!-- Damage Type -->
      <Container class="col-span-4">
        <Label>Damage Type</Label>
        <Select
          bind:value={rangedAttackAction.damageType}
          placeholder="Select a damage type"
          items={damageTypes}
        />
      </Container>

      {#if index !== monster.rangedAttackActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Recharge Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Recharge Actions
    </h2>
    <!-- Add Recharge Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddRechargeAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.rechargeActions as rechargeAction, index}
      <!-- Name -->
      <Container class="col-span-6">
        <Label>Name</Label>
        <Input
          bind:value={rechargeAction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

      <!-- Recharge -->
      <Container class="col-span-3">
        <Label>Recharge</Label>
        <Select
          bind:value={rechargeAction.rechargeDice}
          placeholder="Select a recharge dice"
          items={recharges}
        />
      </Container>

      <!-- Remove Recharge Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveRechargeAction(rechargeAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={rechargeAction.description}
          placeholder="Write a description for the recharge action.."
        />
      </Container>

      {#if index !== monster.rechargeActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Bonus Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Bonus Actions
    </h2>
    <!-- Add Bonus Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddBonusAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.bonusActions as bonusAction, index}
      <!-- Name -->
      <Container class="col-span-9">
        <Label>Name</Label>
        <Input
          bind:value={bonusAction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

      <!-- Remove Bonus Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveBonusAction(bonusAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={bonusAction.description}
          placeholder="Write a description for the bonus action.."
        />
      </Container>

      {#if index !== monster.bonusActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Reactions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Reactions
    </h2>
    <!-- Add Reaction Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddReaction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.reactions as reaction, index}
      <!-- Name -->
      <Container class="col-span-9">
        <Label>Name</Label>
        <Input
          bind:value={reaction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

      <!-- Remove Reaction Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveReaction(reaction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={reaction.description}
          placeholder="Write a description for the reaction.."
        />
      </Container>

      {#if index !== monster.reactions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Legendary Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Legendary Actions
    </h2>
    <!-- Add Legendary Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddLegendaryAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#if monster.legendaryActions.length > 0}
      <Container class="col-span-3">
        <Label>Available Legendary Actions per Turn</Label>
        <Input
          bind:value={monster.availableLegendaryActionsPerTurn}
          type="number"
          placeholder="3"
          class="text-center"
        />
      </Container>
      <div class="col-span-7"></div>
    {/if}

    {#each monster.legendaryActions as legendaryAction, index}
      <!-- Name -->
      <Container class="col-span-7">
        <Label>Name</Label>
        <Input
          bind:value={legendaryAction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

      <!-- Cost -->
      <Container class="col-span-2">
        <Label>Cost</Label>
        <Input
          bind:value={legendaryAction.cost}
          type="number"
          placeholder="3"
          class="text-center"
        />
      </Container>

      <!-- Remove Legendary Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveLegendaryAction(legendaryAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={legendaryAction.description}
          placeholder="Write a description for the reaction.."
        />
      </Container>

      {#if index !== monster.legendaryActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Lair Actions -->
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Lair Actions
    </h2>
    <!-- Add Lair Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e) => monster.AddLairAction(e)}
      >
        <CirclePlus />
      </Button>
    </div>

    {#each monster.lairActions as lairAction, index}
      <!-- Name -->
      <Container class="col-span-9">
        <Label>Name</Label>
        <Input
          bind:value={lairAction.name}
          type="text"
          placeholder="Martial Advantage"
        />
      </Container>

      <!-- Remove Lair Action Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_) => monster.RemoveLairAction(lairAction)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label>Description</Label>
        <TextArea
          bind:value={lairAction.description}
          placeholder="Write a description for the reaction.."
        />
      </Container>

      {#if index !== monster.lairActions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}
  </Tabs.Content>
  <Tabs.Content value="spellcasting" class="mt-5">
    <h2 class="text-muted-foreground col-span-9 mb-2 ml-1 text-xl">
      Spellcasting
    </h2>
  </Tabs.Content>
</Tabs.Root>

<!-- <form class="space-y-2 py-2"> -->
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
