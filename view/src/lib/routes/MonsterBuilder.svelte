<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";

  import * as Tabs from "$components/ui/tabs/index";
  import { Button } from "$components/ui/button/index";

  import {
    Alignments,
    Language,
    Languages,
    MonsterTypes,
    Movements,
    Recharges,
    Sights,
    Sizes,
    Skill,
    Skills,
  } from "$monster/types";
  import { type Spell, SpellLevel, SpellLevels } from "$spell/types";
  import {
    Attribute,
    Attributes,
    Condition,
    Conditions,
    DamageType,
    DamageTypes,
  } from "$shared/types";
  import { MonsterActions } from "$monster/types/Monster";

  import { LabelValueFactory, RecordFactory } from "$shared/utils/factories";
  import { GetAllSpells } from "$spell/service";

  import * as Command from "$components/ui/command/index";
  import Container from "$components/Container.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import Select from "$components/Select.svelte";
  import TextArea from "$components/TextArea.svelte";
  import Title from "$components/Title.svelte";
  import Toggle from "$components/Toggle.svelte";

  const alignments = LabelValueFactory(Alignments);
  const attributes = LabelValueFactory(Attributes);
  const damageTypes = LabelValueFactory(DamageTypes);
  const monsterTypes = LabelValueFactory(MonsterTypes);
  const movements = LabelValueFactory(Movements);
  const recharges = LabelValueFactory(Recharges);
  const sights = LabelValueFactory(Sights);
  const sizes = LabelValueFactory(Sizes);

  let monster = $state(MonsterActions.EmptyMonster());
  let spells: Spell[] = $state([]);
  let spellSlots: Record<SpellLevel, number | undefined> = $state(
    RecordFactory(SpellLevels, undefined),
  );
  let skills: Record<Skill, number | undefined> = $state(
    RecordFactory(Skills, undefined),
  );
  let languages: Record<Language, boolean> = $state(
    RecordFactory(Languages, false),
  );
  let savingThrows: Record<Attribute, number | undefined> = $state(
    RecordFactory(Attributes, undefined),
  );
  let damageResistances: Record<DamageType, boolean> = $state(
    RecordFactory(DamageTypes, false),
  );
  let damageImmunities: Record<DamageType, boolean> = $state(
    RecordFactory(DamageTypes, false),
  );
  let conditionImmunities: Record<Condition, boolean> = $state(
    RecordFactory(Conditions, false),
  );

  const getSpells = async (): Promise<void> => {
    let all_spells = await GetAllSpells();
    spells = all_spells.sort((a, b) => a.name!.localeCompare(b.name!));
  };
</script>

<!-- Creates a section for picking spells to add to a monster. -->
{#snippet SpellPickSection(title: string, spellLevel: SpellLevel)}
  <Command.Group heading={title}>
    {#each spells.filter((spell) => spell.level === spellLevel) as spell}
      <Command.Item
        class="flex justify-between"
        value={spell.name}
        onclick={(_) => monster.spells.push(spell)}
        disabled={monster.spells.includes(spell)}
      >
        <span>{spell.name}</span>
        <span class="text-muted-foreground">{spell.school}</span>
      </Command.Item>
    {/each}
  </Command.Group>
{/snippet}

<!-- Creates a section for showing spells added to a monster. -->
{#snippet SpellListSection(title: string, spellLevel: SpellLevel)}
  <div class="grid h-[50px] grid-cols-10 content-center">
    <h4 class="text-muted-foreground col-span-9 py-2.5 text-xs font-medium">
      {title}
    </h4>
    {#if spellLevel !== SpellLevel.Cantrip}
      <Input
        bind:value={spellSlots[spellLevel]}
        placeholder="3"
        type="number"
        class="col-span-1 text-center"
      />
    {/if}
  </div>
  <div class="space-y-2">
    {#each monster.spells
      .filter((spell) => spell.level === spellLevel)
      .sort((a, b) => a.name!.localeCompare(b.name!)) as spell}
      <div class="flex justify-between text-sm">
        <span>{spell.name}</span>
        <div class="flex space-x-2">
          <span class="text-muted-foreground">{spell.school}</span>
          <CircleX
            class="text-red-300 hover:text-red-600"
            size="18"
            onclick={(_: MouseEvent) =>
              MonsterActions.RemoveSpell(monster, spell)}
          />
        </div>
      </div>
    {/each}
  </div>
{/snippet}

<Tabs.Root value="basic" class="mx-auto mt-5 w-[1000px]">
  <div class="flex w-full justify-center">
    <Tabs.List class="flex w-full justify-center">
      <Tabs.Trigger value="basic">Basic Information</Tabs.Trigger>
      <Tabs.Trigger value="defensive">Defensive</Tabs.Trigger>
      <Tabs.Trigger value="traits">Traits</Tabs.Trigger>
      <Tabs.Trigger value="actions">Actions</Tabs.Trigger>
      <Tabs.Trigger value="spellcasting">Spellcasting</Tabs.Trigger>
    </Tabs.List>
  </div>

  <Tabs.Content value="basic" class="space-y-5">
    <Title variant="muted">Basic Monster Information</Title>
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
    <Title variant="muted">Attributes</Title>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Attributes -->
      <Container class="col-span-3">
        <Label>{Attribute.Strength}</Label>
        <Input
          bind:value={monster.strength}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>

      <Container class="col-span-3">
        <Label>{Attribute.Dexterity}</Label>
        <Input
          bind:value={monster.dexterity}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>

      <Container class="col-span-3">
        <Label>{Attribute.Consitution}</Label>
        <Input
          bind:value={monster.constitution}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>

      <Container class="col-span-3">
        <Label>{Attribute.Intelligence}</Label>
        <Input
          bind:value={monster.intelligence}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>

      <Container class="col-span-3">
        <Label>{Attribute.Wisdom}</Label>
        <Input
          bind:value={monster.wisdom}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>

      <Container class="col-span-3">
        <Label>{Attribute.Charisma}</Label>
        <Input
          bind:value={monster.charisma}
          type="number"
          placeholder="13"
          class="text-center"
        />
      </Container>
    </div>

    <!-- Skills -->
    <Title variant="muted">Skills</Title>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      {#each Skills as skill}
        <Container class="col-span-3">
          <Label>{skill}</Label>
          <Input
            bind:value={skills[skill]}
            type="number"
            placeholder=""
            class="text-center"
          />
        </Container>
      {/each}
    </div>

    <!-- Languages -->
    <Title variant="muted">Languages</Title>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each Languages as language}
        <Container class="col-span-1">
          <Toggle bind:checked={languages[language]}>
            {language}
          </Toggle>
        </Container>
      {/each}
    </div>

    <div class="grid grid-cols-2 space-y-5 gap-x-2">
      <!-- Vision -->
      <div class="col-span-1">
        <div class="grid grid-cols-10 space-y-5 gap-x-2">
          <Title variant="muted">Vision</Title>
          <!-- Add Vission Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <Button
              variant="ghost"
              size="icon"
              class="text-green-300 hover:text-green-600"
              onclick={(e: MouseEvent) => MonsterActions.AddVision(monster, e)}
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
                onclick={(_: MouseEvent) =>
                  MonsterActions.RemoveVision(monster, vision)}
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
          <Title variant="muted">Speed</Title>
          <!-- Add Speed Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <Button
              variant="ghost"
              size="icon"
              class="text-green-300 hover:text-green-600"
              onclick={(e: MouseEvent) => MonsterActions.AddSpeed(monster, e)}
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
                onclick={(_: MouseEvent) =>
                  MonsterActions.RemoveSpeed(monster, speed)}
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
    <Title variant="muted" class="col-span-9">Basic Defensive Information</Title
    >
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
    <Title variant="muted">Saving Throws</Title>
    <div class="grid grid-cols-18 space-y-5 gap-x-2">
      <!-- Saving Throws -->
      {#each Attributes as attribute}
        <Container class="col-span-3">
          <Label>{attribute}</Label>
          <Input
            bind:value={savingThrows[attribute]}
            type="number"
            placeholder=""
            class="text-center"
          />
        </Container>
      {/each}
    </div>

    <!-- Damage Resistances -->
    <Title variant="muted">Damage Resistances</Title>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={damageResistances[damageType]}>
            {damageType}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Damage Immunities -->
    <Title variant="muted">Damage Immunities</Title>
    <div class="grid grid-cols-4 space-y-2 gap-x-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={damageImmunities[damageType]}>
            {damageType}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Condition Immunities -->
    <Title variant="muted">Condition Immunities</Title>
    <div class="grid grid-cols-5 space-y-2 gap-x-2">
      {#each Conditions as condition}
        <Container class="col-span-1">
          <Toggle bind:checked={conditionImmunities[condition]}>
            {condition}
          </Toggle>
        </Container>
      {/each}
    </div>
  </Tabs.Content>
  <Tabs.Content value="traits" class="mt-5">
    <div class="grid grid-cols-10 space-y-5 gap-x-2">
      <!-- Traits -->
      <Title variant="muted" class="col-span-9">Traits</Title>
      <!-- Add Trait Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="w-full text-green-300 hover:text-green-600"
          onclick={(e: MouseEvent) => MonsterActions.AddTrait(monster, e)}
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
            onclick={(_: MouseEvent) =>
              MonsterActions.RemoveTrait(monster, trait)}
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
    <Title variant="muted" class="col-span-9">Regular Actions</Title>
    <!-- Add Regular Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) => MonsterActions.AddRegularAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveRegularAction(monster, regularAction)}
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
    <Title variant="muted" class="col-span-9">Melee Attack Actions</Title>
    <!-- Add Melee Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) =>
          MonsterActions.AddMeleeAttackAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveMeleeAttackAction(monster, meleeAttackAction)}
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
    <Title variant="muted" class="col-span-9">Ranged Attack Actions</Title>
    <!-- Add Ranged Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) =>
          MonsterActions.AddRangedAttackAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveRangedAttackAction(
              monster,
              rangedAttackAction,
            )}
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
    <Title variant="muted" class="col-span-9">Recharge Actions</Title>
    <!-- Add Recharge Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) =>
          MonsterActions.AddRechargeAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveRechargeAction(monster, rechargeAction)}
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
    <Title variant="muted" class="col-span-9">Bonus Actions</Title>
    <!-- Add Bonus Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) => MonsterActions.AddBonusAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveBonusAction(monster, bonusAction)}
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
    <Title variant="muted" class="col-span-9">Reactions</Title>
    <!-- Add Reaction Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) => MonsterActions.AddReaction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveReaction(monster, reaction)}
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
    <Title variant="muted" class="col-span-9">Legendary Actions</Title>
    <!-- Add Legendary Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) =>
          MonsterActions.AddLegendaryAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveLegendaryAction(monster, legendaryAction)}
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
    <Title variant="muted" class="col-span-9">Lair Actions</Title>
    <!-- Add Lair Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <Button
        variant="ghost"
        size="icon"
        class="w-full text-green-300 hover:text-green-600"
        onclick={(e: MouseEvent) => MonsterActions.AddLairAction(monster, e)}
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
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveLairAction(monster, lairAction)}
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

  <Tabs.Content
    value="spellcasting"
    class="mt-5 grid grid-cols-16 space-y-5 gap-x-2"
  >
    <Title variant="muted" class="col-span-16">Spellcasting</Title>
    <!-- Level -->
    <Container class="col-span-4">
      <Label>Level</Label>
      <Input
        bind:value={monster.spellcastingLevel}
        type="number"
        class="text-center"
        placeholder="15"
      />
    </Container>

    <!-- Attribute -->
    <Container class="col-span-4">
      <Label>Attribute</Label>
      <Select
        bind:value={monster.spellcastingAttribute}
        placeholder="Select an attribute"
        items={attributes}
      />
    </Container>

    <!-- Saving Throw DC -->
    <Container class="col-span-4">
      <Label>Saving Throw DC</Label>
      <Input
        bind:value={monster.spellcastingDC}
        type="number"
        class="text-center"
        placeholder="18"
      />
    </Container>

    <!-- Attack Bonus -->
    <Container class="col-span-4">
      <Label>Attack Bonus</Label>
      <Input
        bind:value={monster.spellcastingAttackBonus}
        type="number"
        class="text-center"
        placeholder="9"
      />
    </Container>

    <Title variant="muted" class="col-span-16">Spell List</Title>

    {#await getSpells()}
      Loading...
    {:then}
      <Command.Root class="col-span-8 h-[300px] w-full rounded-lg border">
        <Command.Input placeholder="Search for a spell" />
        <Command.List>
          <Command.Empty>No results found.</Command.Empty>
          {@render SpellPickSection("Cantrips", SpellLevel.Cantrip)}
          <Command.Separator />
          {@render SpellPickSection("1st level", SpellLevel.First)}
          <Command.Separator />
          {@render SpellPickSection("2nd level", SpellLevel.Second)}
          <Command.Separator />
          {@render SpellPickSection("3rd level", SpellLevel.Third)}
          <Command.Separator />
          {@render SpellPickSection("4th level", SpellLevel.Fourth)}
          <Command.Separator />
          {@render SpellPickSection("5th level", SpellLevel.Fifth)}
          <Command.Separator />
          {@render SpellPickSection("6th level", SpellLevel.Sixth)}
          <Command.Separator />
          {@render SpellPickSection("7th level", SpellLevel.Seventh)}
          <Command.Separator />
          {@render SpellPickSection("8th level", SpellLevel.Eighth)}
          <Command.Separator />
          {@render SpellPickSection("9th level", SpellLevel.Ninth)}
        </Command.List>
      </Command.Root>
    {:catch error}
      Some {error} occurred.
    {/await}

    <ScrollArea
      class="col-span-8 h-[300px] w-full rounded-md border"
      scrollbarYClasses="hidden"
    >
      <div class="p-2">
        {@render SpellListSection("Cantrips", SpellLevel.Cantrip)}
        <Separator />

        {@render SpellListSection("1st level", SpellLevel.First)}
        <Separator />

        {@render SpellListSection("2nd level", SpellLevel.Second)}
        <Separator />

        {@render SpellListSection("3rd level", SpellLevel.Third)}
        <Separator />

        {@render SpellListSection("4th level", SpellLevel.Fourth)}
        <Separator />

        {@render SpellListSection("5th level", SpellLevel.Fifth)}
        <Separator />

        {@render SpellListSection("6th level", SpellLevel.Sixth)}
        <Separator />

        {@render SpellListSection("7th level", SpellLevel.Seventh)}
        <Separator />

        {@render SpellListSection("8th level", SpellLevel.Eighth)}
        <Separator />

        {@render SpellListSection("9th level", SpellLevel.Ninth)}
      </div>
    </ScrollArea>
  </Tabs.Content>
</Tabs.Root>
