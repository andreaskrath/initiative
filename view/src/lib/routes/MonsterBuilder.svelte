<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import CircleX from "@lucide/svelte/icons/circle-x";

  import * as Tabs from "$components/ui/tabs/index";
  import { Button } from "$components/ui/button/index";

  import {
    Alignments,
    Attribute,
    Attributes,
    Condition,
    Conditions,
    DamageType,
    DamageTypes,
    DisplayAlignment,
    DisplayAttribute,
    DisplayCondition,
    DisplayDamageType,
    DisplayLanguage,
    DisplayMagicSchool,
    DisplayMonsterType,
    DisplayMovement,
    DisplaySight,
    DisplaySize,
    DisplaySkill,
    DisplayTrigger,
    Language,
    Languages,
    MonsterActions,
    MonsterTypes,
    Movements,
    Recharges,
    Sights,
    Sizes,
    Skill,
    Skills,
    SpellLevel,
    SpellLevels,
    TurnTriggers,
    type Spell,
  } from "$types";

  import {
    ToLabelValue,
    ToLabelValueWith,
    RecordFactory,
  } from "$shared/utils/factories";
  import { GetAllSpells } from "$spell/service";

  import * as Command from "$components/ui/command/index";
  import Container from "$components/Container.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import Select from "$components/Select.svelte";
  import { Skeleton } from "$lib/shared/components/ui/skeleton/index";
  import TextArea from "$components/TextArea.svelte";
  import Title from "$components/Title.svelte";
  import Toggle from "$components/Toggle.svelte";
  import { MonsterService } from "$lib/monster/service";
  import { StatusCodes } from "http-status-codes";
  import { toast } from "svelte-sonner";
  import { goto } from "@mateothegreat/svelte5-router";
  import Combobox from "$lib/shared/components/Combobox.svelte";
  import type { FieldErrors } from "$utils/error";
  import AddButton from "$components/AddButton.svelte";

  const alignments = ToLabelValueWith(Alignments, DisplayAlignment);
  const attributes = ToLabelValueWith(Attributes, DisplayAttribute);
  const damageTypes = ToLabelValueWith(DamageTypes, DisplayDamageType);
  const monsterTypes = ToLabelValueWith(MonsterTypes, DisplayMonsterType);
  const movements = ToLabelValueWith(Movements, DisplayMovement);
  const recharges = ToLabelValue(Recharges);
  const sights = ToLabelValueWith(Sights, DisplaySight);
  const sizes = ToLabelValueWith(Sizes, DisplaySize);
  const turnTriggers = ToLabelValueWith(TurnTriggers, DisplayTrigger);

  let monster = $state(MonsterActions.EmptyMonster());
  let spells: Spell[] = $state([]);
  let errors: FieldErrors | null = $state(null);

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

  const handleCreateMonster = async (event: MouseEvent): Promise<void> => {
    event.preventDefault();

    monster.spellcasting!.spell_slots = [];
    for (const spellLevel of SpellLevels) {
      let value = spellSlots[spellLevel];
      if (value) {
        monster.spellcasting!.spell_slots.push({
          level: spellLevel,
          slots: value,
        });
      }
    }

    monster.skills = [];
    for (const skill of Skills) {
      let value = skills[skill];
      if (value) {
        monster.skills.push({
          skill: skill,
          modifier: value,
        });
      }
    }

    monster.saving_throws = [];
    for (const attribute of Attributes) {
      let value = savingThrows[attribute];
      if (value) {
        monster.saving_throws.push({
          attribute: attribute,
          modifier: value,
        });
      }
    }

    monster.languages = [];
    for (const language of Languages) {
      if (languages[language]) {
        monster.languages.push(language);
      }
    }

    monster.damage_resistances = [];
    monster.damage_immunities = [];
    for (const damageType of DamageTypes) {
      if (damageResistances[damageType]) {
        monster.damage_resistances.push(damageType);
      }

      if (damageImmunities[damageType]) {
        monster.damage_immunities.push(damageType);
      }
    }

    monster.condition_immunities = [];
    for (const condition of Conditions) {
      if (conditionImmunities[condition]) {
        monster.condition_immunities.push(condition);
      }
    }

    const result = await MonsterService.Create(monster);
    if (typeof result === "number") {
      errors = null;
      switch (result) {
        case StatusCodes.CREATED:
          toast.success("Successfully created monster");
          goto("/monsters");
          break;
        case StatusCodes.CONFLICT:
          toast.error("A monster with this name already exists");
          break;
        case StatusCodes.INTERNAL_SERVER_ERROR:
          toast.success("Internal server error");
          break;
        default:
          toast.error("An unknown error occured");
      }
    } else {
      errors = result;
      toast.error("There is an issue with the defined monster");
    }
  };
</script>

<!-- Creates a section for picking spells to add to a monster. -->
{#snippet SpellPickSection(title: string, spellLevel: SpellLevel)}
  <Command.Group heading={title}>
    {#each spells.filter((spell) => spell.level === spellLevel) as spell}
      <Command.Item
        class="flex justify-between"
        value={spell.name}
        onclick={(_) => monster.spellcasting!.spells.push(spell)}
        disabled={monster.spellcasting!.spells.includes(spell)}
      >
        <span>{spell.name}</span>
        <span class="text-muted-foreground">
          {DisplayMagicSchool[spell.school!]}
        </span>
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
    {#each monster
      .spellcasting!.spells.filter((spell) => spell.level === spellLevel)
      .sort((a, b) => a.name!.localeCompare(b.name!)) as spell}
      <div class="flex justify-between text-sm">
        <span>{spell.name}</span>
        <div class="flex gap-x-2">
          <span class="text-muted-foreground">
            {DisplayMagicSchool[spell.school!]}
          </span>
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
      <Tabs.Trigger value="traits">Traits & Reminders</Tabs.Trigger>
      <Tabs.Trigger value="actions">Actions</Tabs.Trigger>
      <Tabs.Trigger value="spellcasting">Spellcasting</Tabs.Trigger>
    </Tabs.List>
  </div>

  <Tabs.Content value="basic" class="space-y-5">
    <Title variant="muted">Basic Monster Information</Title>
    <div class="grid grid-cols-16 gap-x-2 gap-y-5">
      <!-- Name -->
      <Container class="col-span-7">
        <Label required>Name</Label>
        <Input
          bind:value={monster.name}
          type="text"
          placeholder="Goblin"
          error={errors?.get("name")}
        />
      </Container>

      <!-- Challenge Rating -->
      <Container class="col-span-3">
        <Label required>Challenge Rating</Label>
        <Input
          bind:value={monster.challenge_rating}
          type="number"
          placeholder="0.5"
          class="text-center"
          error={errors?.get("challenge_rating")}
        />
      </Container>

      <!-- XP -->
      <Container class="col-span-3">
        <Label required>Experience Points</Label>
        <Input
          bind:value={monster.xp}
          type="number"
          placeholder="100"
          class="text-center"
          error={errors?.get("xp")}
        />
      </Container>

      <!-- Proficiency Bonus -->
      <Container class="col-span-3">
        <Label required>Proficiency Bonus</Label>
        <Input
          bind:value={monster.proficiency_bonus}
          type="number"
          placeholder="2"
          class="text-center"
          error={errors?.get("proficiency_bonus")}
        />
      </Container>

      <!-- Species -->
      <Container class="col-span-4">
        <Label>Species</Label>
        <Input
          bind:value={monster.species}
          type="text"
          placeholder="Goblinoid"
          error={errors?.get("species")}
        />
      </Container>

      <!-- Type -->
      <Container class="col-span-3">
        <Label required>Type</Label>
        <Select
          bind:value={monster.monster_type}
          placeholder="Select a type"
          items={monsterTypes}
          error={errors?.get("monster_type")}
        />
      </Container>

      <!-- Size -->
      <Container class="col-span-3">
        <Label required>Size</Label>
        <Select
          bind:value={monster.size}
          placeholder="Select a size"
          items={sizes}
          error={errors?.get("size")}
        />
      </Container>

      <!-- Alignment -->
      <Container class="col-span-3">
        <Label required>Alignment</Label>
        <Select
          bind:value={monster.alignment}
          placeholder="Select an alignment"
          items={alignments}
          error={errors?.get("alignment")}
        />
      </Container>

      <!-- Passive Perception -->
      <Container class="col-span-3">
        <Label required>Passive Perception</Label>
        <Input
          bind:value={monster.passive_perception}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("passive_perception")}
        />
      </Container>
    </div>

    <!-- Attributes -->
    <Title variant="muted">Attributes</Title>
    <div class="grid grid-cols-18 gap-x-2 gap-y-5">
      <!-- Attributes -->
      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Strength]}</Label>
        <Input
          bind:value={monster.strength}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("strength")}
        />
      </Container>

      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Dexterity]}</Label>
        <Input
          bind:value={monster.dexterity}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("dexterity")}
        />
      </Container>

      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Constitution]}</Label>
        <Input
          bind:value={monster.constitution}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("constitution")}
        />
      </Container>

      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Intelligence]}</Label>
        <Input
          bind:value={monster.intelligence}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("intelligence")}
        />
      </Container>

      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Wisdom]}</Label>
        <Input
          bind:value={monster.wisdom}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("wisdom")}
        />
      </Container>

      <Container class="col-span-3">
        <Label required>{DisplayAttribute[Attribute.Charisma]}</Label>
        <Input
          bind:value={monster.charisma}
          type="number"
          placeholder="13"
          class="text-center"
          error={errors?.get("charisma")}
        />
      </Container>
    </div>

    <!-- Skills -->
    <Title variant="muted">Skills</Title>
    <div class="grid grid-cols-18 gap-x-2 gap-y-5">
      {#each Skills as skill}
        <Container class="col-span-3">
          <Label>{DisplaySkill[skill]}</Label>
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
    <div class="grid grid-cols-4 gap-x-2 gap-y-2">
      {#each Languages as language}
        <Container class="col-span-1">
          <Toggle bind:checked={languages[language]}>
            {DisplayLanguage[language]}
          </Toggle>
        </Container>
      {/each}
    </div>

    <div class="grid grid-cols-2 gap-x-2 gap-y-5">
      <!-- Vision -->
      <div class="col-span-1">
        <div class="grid grid-cols-10 gap-x-2 gap-y-5">
          <Title variant="muted">Vision</Title>
          <!-- Add Vission Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <AddButton onclick={(_) => MonsterActions.AddVision(monster)} />
          </div>

          <!-- Vision List -->
          {#each monster.visions as vision, index (vision)}
            <!-- Range -->
            <Container class="col-span-3">
              <Label required>Range</Label>
              <Input
                bind:value={vision.range}
                type="number"
                placeholder="60"
                class="text-center"
                error={errors?.get(`visions.${index}.range`)}
              />
            </Container>

            <!-- Sight Type -->
            <Container class="col-span-6">
              <Label required>Sight</Label>
              <Select
                bind:value={vision.sight}
                placeholder="Select a vision type"
                items={sights}
                error={errors?.get(`visions.${index}.sight`)}
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
        <div class="grid grid-cols-10 gap-x-2 gap-y-5">
          <Title variant="muted">Speed</Title>
          <!-- Add Speed Button -->
          <div class="col-span-1 col-start-10 flex justify-center">
            <AddButton onclick={(_) => MonsterActions.AddSpeed(monster)} />
          </div>

          <!-- Speed List -->
          {#each monster.speeds as speed, index (speed)}
            <!-- Distance -->
            <Container class="col-span-3">
              <Label required>Distance</Label>
              <Input
                bind:value={speed.distance}
                type="number"
                placeholder="30"
                class="text-center"
                error={errors?.get(`speeds.${index}.distance`)}
              />
            </Container>

            <!-- Movement Type -->
            <Container class="col-span-6">
              <Label required>Movement</Label>
              <Select
                bind:value={speed.movement}
                placeholder="Select a movement type"
                items={movements}
                error={errors?.get(`speeds.${index}.movement`)}
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
  <Tabs.Content value="defensive" class="mt-5 space-y-5">
    <Title variant="muted" class="col-span-9">
      Basic Defensive Information
    </Title>
    <div class="grid grid-cols-16 gap-x-2 gap-y-5">
      <!-- Hit Points -->
      <Container class="col-span-3">
        <Label required>Hit Points</Label>
        <Input
          bind:value={monster.hit_points}
          type="number"
          placeholder="11"
          class="text-center"
          error={errors?.get("hit_points")}
        />
      </Container>

      <!-- Rollable Hit Points -->
      <Container class="col-span-3">
        <Label required>Rollable Hit Points</Label>
        <Input
          bind:value={monster.rollable_hit_points}
          type="text"
          placeholder="2d8 + 6"
          class="text-center"
          error={errors?.get("rollable_hit_points")}
        />
      </Container>

      <!-- Armor Class -->
      <Container class="col-span-3">
        <Label required>Armor Class</Label>
        <Input
          bind:value={monster.armor_class}
          type="number"
          placeholder="18"
          class="text-center"
          error={errors?.get("armor_class")}
        />
      </Container>

      <!-- Armor Type -->
      <Container class="col-span-7">
        <Label>Armor Type</Label>
        <Input
          bind:value={monster.armor_type}
          type="text"
          placeholder="chain mail, shield"
          error={errors?.get("armor_type")}
        />
      </Container>
    </div>

    <!-- Saving Throws -->
    <Title variant="muted">Saving Throws</Title>
    <div class="grid grid-cols-18 gap-x-2 gap-y-5">
      <!-- Saving Throws -->
      {#each Attributes as attribute}
        <Container class="col-span-3">
          <Label>{DisplayAttribute[attribute]}</Label>
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
    <div class="grid grid-cols-4 gap-x-2 gap-y-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={damageResistances[damageType]}>
            {DisplayDamageType[damageType]}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Damage Immunities -->
    <Title variant="muted">Damage Immunities</Title>
    <div class="grid grid-cols-4 gap-x-2 gap-y-2">
      {#each DamageTypes as damageType}
        <Container class="col-span-1">
          <Toggle bind:checked={damageImmunities[damageType]}>
            {DisplayDamageType[damageType]}
          </Toggle>
        </Container>
      {/each}
    </div>

    <!-- Condition Immunities -->
    <Title variant="muted">Condition Immunities</Title>
    <div class="grid grid-cols-5 gap-x-2 gap-y-2">
      {#each Conditions as condition}
        <Container class="col-span-1">
          <Toggle bind:checked={conditionImmunities[condition]}>
            {DisplayCondition[condition]}
          </Toggle>
        </Container>
      {/each}
    </div>
  </Tabs.Content>
  <Tabs.Content value="traits" class="mt-5 grid grid-cols-10 gap-x-2 gap-y-5">
    <!-- Traits -->
    <Title variant="muted" class="col-span-9">Traits</Title>
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddTrait(monster)} />
    </div>

    {#each monster.traits as trait, index (trait)}
      <!-- Name -->
      <Container class="col-span-9">
        <Label required>Name</Label>
        <Input
          bind:value={trait.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`traits.${index}.name`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={trait.description}
          placeholder="Write a description for the trait.."
          error={errors?.get(`traits.${index}.description`)}
        />
      </Container>

      {#if index !== monster.traits.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Reminders -->
    <Title variant="muted" class="col-span-9">Reminders</Title>
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddReminder(monster)} />
    </div>

    {#each monster.reminders as reminder, index (reminder)}
      <!-- Name -->
      <Container class="col-span-4">
        <Label required>Name</Label>
        <Input
          bind:value={reminder.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`reminders.${index}.name`)}
        />
      </Container>

      <!-- Trigger Type -->
      <Container class="col-span-5">
        <Label required>Trigger</Label>
        <Select
          bind:value={reminder.trigger}
          placeholder="Select a trigger type"
          items={turnTriggers}
          error={errors?.get(`reminders.${index}.trigger`)}
        />
      </Container>

      <!-- Remove Reminder Button -->
      <div class="col-span-1 col-start-10 flex justify-center">
        <Button
          variant="ghost"
          size="icon"
          class="mt-5 w-full text-red-300 hover:text-red-600"
          onclick={(_: MouseEvent) =>
            MonsterActions.RemoveReminder(monster, reminder)}
        >
          <CircleX />
        </Button>
      </div>

      <!-- Description -->
      <Container class="col-span-10">
        <Label required>Description</Label>
        <TextArea
          bind:value={reminder.description}
          placeholder="Write a description for the reminder.."
          error={errors?.get(`reminders.${index}.description`)}
        />
      </Container>

      {#if index !== monster.reminders.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}
  </Tabs.Content>
  <Tabs.Content value="actions" class="mt-5 grid grid-cols-10 gap-x-2 gap-y-5">
    <!-- Regular Actions -->
    <Title variant="muted" class="col-span-9">Regular Actions</Title>
    <!-- Add Regular Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddRegularAction(monster)} />
    </div>

    {#each monster.regular_actions as regularAction, index (regularAction)}
      <!-- Name -->
      <Container class="col-span-9">
        <Label required>Name</Label>
        <Input
          bind:value={regularAction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`regular_actions.${index}.name`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={regularAction.description}
          placeholder="Write a description for the regular action.."
          error={errors?.get(`regular_actions.${index}.description`)}
        />
      </Container>

      {#if index !== monster.regular_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Melee Attack Actions -->
    <Title variant="muted" class="col-span-9">Melee Attack Actions</Title>
    <!-- Add Melee Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton
        onclick={(_) => MonsterActions.AddMeleeAttackAction(monster)}
      />
    </div>

    {#each monster.melee_attack_actions as meleeAttackAction, index (meleeAttackAction)}
      <!-- Name -->
      <Container class="col-span-5">
        <Label required>Name</Label>
        <Input
          bind:value={meleeAttackAction.name}
          type="text"
          placeholder="Longsword"
          error={errors?.get(`melee_attack_actions.${index}.name`)}
        />
      </Container>

      <!-- Bonus to Hit -->
      <Container class="col-span-2">
        <Label required>Bonus to Hit</Label>
        <Input
          bind:value={meleeAttackAction.hit_bonus}
          type="number"
          placeholder="5"
          class="text-center"
          error={errors?.get(`melee_attack_actions.${index}.hit_bonus`)}
        />
      </Container>

      <!-- Reach -->
      <Container class="col-span-2">
        <Label required>Reach</Label>
        <Input
          bind:value={meleeAttackAction.reach}
          type="number"
          placeholder="5"
          class="text-center"
          error={errors?.get(`melee_attack_actions.${index}.reach`)}
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
          bind:value={meleeAttackAction.one_handed_attack}
          type="text"
          placeholder="1d8 + 1"
          class="text-center"
          error={errors?.get(`melee_attack_actions.${index}.one_handed_attack`)}
        />
      </Container>

      <!-- Two-Handed Attack -->
      <Container class="col-span-3">
        <Label>Two-Handed Attack</Label>
        <Input
          bind:value={meleeAttackAction.two_handed_attack}
          type="text"
          placeholder="1d10 + 1"
          class="text-center"
          error={errors?.get(`melee_attack_actions.${index}.two_handed_attack`)}
        />
      </Container>

      <!-- Damage Type -->
      <Container class="col-span-4">
        <Label required>Damage Type</Label>
        <Select
          bind:value={meleeAttackAction.damage_type}
          placeholder="Select a damage type"
          items={damageTypes}
          error={errors?.get(`melee_attack_actions.${index}.damage_type`)}
        />
      </Container>

      {#if index !== monster.melee_attack_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Ranged Attack Actions -->
    <Title variant="muted" class="col-span-9">Ranged Attack Actions</Title>
    <!-- Add Ranged Attack Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton
        onclick={(_) => MonsterActions.AddRangedAttackAction(monster)}
      />
    </div>

    {#each monster.ranged_attack_actions as rangedAttackAction, index (rangedAttackAction)}
      <!-- Name -->
      <Container class="col-span-5">
        <Label required>Name</Label>
        <Input
          bind:value={rangedAttackAction.name}
          type="text"
          placeholder="Longbow"
          error={errors?.get(`ranged_attack_actions.${index}.name`)}
        />
      </Container>

      <!-- Bonus to Hit -->
      <Container class="col-span-2">
        <Label required>Bonus to Hit</Label>
        <Input
          bind:value={rangedAttackAction.hit_bonus}
          type="number"
          placeholder="5"
          class="text-center"
          error={errors?.get(`ranged_attack_actions.${index}.hit_bonus`)}
        />
      </Container>

      <!-- Attack -->
      <Container class="col-span-2">
        <Label required>Attack</Label>
        <Input
          bind:value={rangedAttackAction.attack}
          type="text"
          placeholder="1d8 + 2"
          class="text-center"
          error={errors?.get(`ranged_attack_actions.${index}.attack`)}
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
        <Label required>Normal Range</Label>
        <Input
          bind:value={rangedAttackAction.normal_range}
          type="number"
          placeholder="150"
          class="text-center"
          error={errors?.get(`ranged_attack_actions.${index}.normal_range`)}
        />
      </Container>

      <!-- Long Range -->
      <Container class="col-span-3">
        <Label required>Long Range</Label>
        <Input
          bind:value={rangedAttackAction.long_range}
          type="number"
          placeholder="600"
          class="text-center"
          error={errors?.get(`ranged_attack_actions.${index}.long_range`)}
        />
      </Container>

      <!-- Damage Type -->
      <Container class="col-span-4">
        <Label required>Damage Type</Label>
        <Select
          bind:value={rangedAttackAction.damage_type}
          placeholder="Select a damage type"
          items={damageTypes}
          error={errors?.get(`ranged_attack_actions.${index}.damage_type`)}
        />
      </Container>

      {#if index !== monster.ranged_attack_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Recharge Actions -->
    <Title variant="muted" class="col-span-9">Recharge Actions</Title>
    <!-- Add Recharge Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddRechargeAction(monster)} />
    </div>

    {#each monster.recharge_actions as rechargeAction, index (rechargeAction)}
      <!-- Name -->
      <Container class="col-span-6">
        <Label required>Name</Label>
        <Input
          bind:value={rechargeAction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`recharge_actions.${index}.name`)}
        />
      </Container>

      <!-- Recharge -->
      <Container class="col-span-3">
        <Label required>Recharge</Label>
        <Combobox
          bind:value={rechargeAction.recharge}
          placeholder="Select a recharge dice"
          items={recharges}
          error={errors?.get(`recharge_actions.${index}.recharge`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={rechargeAction.description}
          placeholder="Write a description for the recharge action.."
          error={errors?.get(`recharge_actions.${index}.description`)}
        />
      </Container>

      {#if index !== monster.regular_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Bonus Actions -->
    <Title variant="muted" class="col-span-9">Bonus Actions</Title>
    <!-- Add Bonus Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddBonusAction(monster)} />
    </div>

    {#each monster.bonus_actions as bonusAction, index (bonusAction)}
      <!-- Name -->
      <Container class="col-span-9">
        <Label required>Name</Label>
        <Input
          bind:value={bonusAction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`bonus_actions.${index}.name`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={bonusAction.description}
          placeholder="Write a description for the bonus action.."
          error={errors?.get(`bonus_actions.${index}.description`)}
        />
      </Container>

      {#if index !== monster.bonus_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Reactions -->
    <Title variant="muted" class="col-span-9">Reactions</Title>
    <!-- Add Reaction Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddReaction(monster)} />
    </div>

    {#each monster.reactions as reaction, index (reaction)}
      <!-- Name -->
      <Container class="col-span-9">
        <Label required>Name</Label>
        <Input
          bind:value={reaction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`reactions.${index}.name`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={reaction.description}
          placeholder="Write a description for the reaction.."
          error={errors?.get(`reactions.${index}.description`)}
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
      <AddButton onclick={(_) => MonsterActions.AddLegendaryAction(monster)} />
    </div>

    {#if monster.legendary_actions.length > 0}
      <Container class="col-span-3">
        <Label required>Available Legendary Actions per Turn</Label>
        <Input
          bind:value={monster.available_legendary_actions_per_turn}
          type="number"
          placeholder="3"
          class="text-center"
          error={errors?.get("available_legendary_actions_per_turn")}
        />
      </Container>
      <div class="col-span-7"></div>
    {/if}

    {#each monster.legendary_actions as legendaryAction, index (legendaryAction)}
      <!-- Name -->
      <Container class="col-span-7">
        <Label required>Name</Label>
        <Input
          bind:value={legendaryAction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`legendary_actions.${index}.name`)}
        />
      </Container>

      <!-- Cost -->
      <Container class="col-span-2">
        <Label required>Cost</Label>
        <Input
          bind:value={legendaryAction.cost}
          type="number"
          placeholder="3"
          class="text-center"
          error={errors?.get(`legendary_actions.${index}.cost`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={legendaryAction.description}
          placeholder="Write a description for the reaction.."
          error={errors?.get(`legendary_actions.${index}.description`)}
        />
      </Container>

      {#if index !== monster.legendary_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}

    <!-- Lair Actions -->
    <Title variant="muted" class="col-span-9">Lair Actions</Title>
    <!-- Add Lair Action Button -->
    <div class="col-span-1 col-start-10 flex justify-center">
      <AddButton onclick={(_) => MonsterActions.AddLairAction(monster)} />
    </div>

    {#each monster.lair_actions as lairAction, index (lairAction)}
      <!-- Name -->
      <Container class="col-span-9">
        <Label required>Name</Label>
        <Input
          bind:value={lairAction.name}
          type="text"
          placeholder="Martial Advantage"
          error={errors?.get(`lair_actions.${index}.name`)}
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
        <Label required>Description</Label>
        <TextArea
          bind:value={lairAction.description}
          placeholder="Write a description for the reaction.."
          error={errors?.get(`lair_actions.${index}.description`)}
        />
      </Container>

      {#if index !== monster.lair_actions.length - 1}
        <hr class="col-span-10" />
      {/if}
    {/each}
  </Tabs.Content>

  <Tabs.Content
    value="spellcasting"
    class="mt-5 grid grid-cols-16 gap-x-2 gap-y-5"
  >
    <Title variant="muted" class="col-span-16">Spellcasting</Title>
    <!-- Level -->
    <Container class="col-span-4">
      <Label>Level</Label>
      <Input
        bind:value={monster.spellcasting!.level}
        type="number"
        class="text-center"
        placeholder="15"
        error={errors?.get("spellcasting.level")}
      />
    </Container>

    <!-- Attribute -->
    <Container class="col-span-4">
      <Label>Attribute</Label>
      <Select
        bind:value={monster.spellcasting!.attribute}
        placeholder="Select an attribute"
        items={attributes}
        error={errors?.get("spellcasting.attribute")}
      />
    </Container>

    <!-- Saving Throw DC -->
    <Container class="col-span-4">
      <Label>Saving Throw DC</Label>
      <Input
        bind:value={monster.spellcasting!.dc}
        type="number"
        class="text-center"
        placeholder="18"
        error={errors?.get("spellcasting.dc")}
      />
    </Container>

    <!-- Attack Bonus -->
    <Container class="col-span-4">
      <Label>Attack Bonus</Label>
      <Input
        bind:value={monster.spellcasting!.attack_bonus}
        type="number"
        class="text-center"
        placeholder="9"
        error={errors?.get("spellcasting.attack_bonus")}
      />
    </Container>

    <Title variant="muted" class="col-span-16">Spell List</Title>

    {#await getSpells()}
      <div class="col-span-8 h-[300px] w-full rounded-lg border">
        <div class="mt-3 ml-5 grid gap-x-4 gap-y-4">
          <Skeleton class="h-4 w-[450px]" />
          <Skeleton class="h-4 w-[400px]" />
          <Skeleton class="h-4 w-[450px]" />
          <Skeleton class="h-4 w-[450px]" />
          <Skeleton class="h-4 w-[400px]" />
          <Skeleton class="h-4 w-[400px]" />
          <Skeleton class="h-4 w-[450px]" />
          <Skeleton class="h-4 w-[450px]" />
          <Skeleton class="h-4 w-[450px]" />
        </div>
      </div>
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
    {:catch _}
      <div class="col-span-8 h-[300px] w-full rounded-lg border">
        <p class="mt-5 text-center">
          An error occurred while loading the spells.
        </p>
      </div>
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

<div class="h-[100px]"></div>

<!-- Create Monster Button -->
<div class="fixed inset-x-0 bottom-0 mx-auto flex w-[1000px] justify-end pb-10">
  <Button onclick={async (e: MouseEvent) => handleCreateMonster(e)}>
    Create Monster
  </Button>
</div>
