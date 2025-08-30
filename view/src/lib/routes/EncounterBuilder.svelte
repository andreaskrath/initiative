<script lang="ts">
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import {
    Attributes,
    CombatEntityActions,
    ConditionsNoExhaustion,
    DisplayAttribute,
    DisplayCondition,
    DisplayExhaustionLevel,
    DisplayMonsterType,
    DisplaySaveTrigger,
    EncounterActions,
    ExhaustionLevel,
    ExhaustionLevels,
    MonsterActions,
    MonsterType,
    MonsterTypes,
    PlayerEntityActions,
    SaveTriggers,
    type CombatCondition,
    type CombatEntity,
    type EncounterEntity,
    type Monster,
    type PlayerEntity,
  } from "$types";

  import { Button } from "$components/ui/button/index";
  import * as Command from "$components/ui/command/index";
  import * as DropdownMenu from "$components/ui/dropdown-menu/index";
  import Label from "$components/Label.svelte";
  import Input from "$components/Input.svelte";
  import Title from "$components/Title.svelte";
  import Container from "$components/Container.svelte";
  import Toggle from "$components/Toggle.svelte";
  import Dialog from "$components/Dialog.svelte";
  import Select from "$components/Select.svelte";
  import { ToLabelValueWith } from "$utils/factories";
  import Combobox from "$components/Combobox.svelte";
  import { ValidatePlayerEntity } from "$encounter/validate";

  let addPlayerDialogOpen = $state(false);

  let encounter = $state(EncounterActions.EmptyEncounter());
  let playerEntities = $derived(
    encounter.entities.filter((entity) => entity.type === "player"),
  );
  let monsterEntities = $derived(
    encounter.entities.filter((entity) => entity.type === "monster"),
  );
  let reminderEntities = $derived(
    encounter.entities.filter((entity) => entity.type === "reminder"),
  );

  let monsters: Monster[] = $state([]);

  const selectConditions = ToLabelValueWith(
    ConditionsNoExhaustion,
    DisplayCondition,
  );
  const selectAttributes = ToLabelValueWith(Attributes, DisplayAttribute);
  const selectSaveTriggers = ToLabelValueWith(SaveTriggers, DisplaySaveTrigger);
  const selectExhaustLevels = ToLabelValueWith(
    ExhaustionLevels,
    DisplayExhaustionLevel,
  );

  let playerForm: PlayerEntity = $state(
    PlayerEntityActions.EmptyPlayerEntity(),
  );

  encounter.entities.push({
    id: "player1",
    name: "Roberto",
    initiative: 15,
    is_active: false,
    concentration: true,
    type: "player",
    conditions: [],
    exhaustion_level: ExhaustionLevel.None,
  });
  encounter.entities.push({
    id: "player2",
    name: "Dixie",
    initiative: 4,
    is_active: false,
    concentration: false,
    type: "player",
    conditions: [],
    exhaustion_level: ExhaustionLevel.None,
  });

  encounter.entities.push({
    id: "monster1",
    name: "Goblin #1",
    initiative: 4,
    is_active: false,
    current_hp: 50,
    max_hp: 60,
    temporary_hp: 20,
    concentration: true,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
    conditions: [],
    exhaustion_level: ExhaustionLevel.None,
  });
  encounter.entities.push({
    id: "monster2",
    name: "Goblin #2",
    initiative: 5,
    is_active: false,
    current_hp: 60,
    max_hp: 160,
    temporary_hp: 0,
    concentration: false,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
    conditions: [],
    exhaustion_level: ExhaustionLevel.None,
  });
  encounter.entities.push({
    id: "monster3",
    name: "Goblin #3",
    initiative: 8,
    is_active: false,
    current_hp: 10,
    max_hp: 160,
    temporary_hp: 0,
    concentration: false,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
    conditions: [],
    exhaustion_level: ExhaustionLevel.None,
  });

  encounter.entities.push({
    id: "reminder1",
    name: "House",
    initiative: 20,
    is_active: false,
    type: "reminder",
    description:
      "The house is falling apart and two random squares on the battle map become holes that you can fall through.",
    win_initiative_tie: false,
  });
  encounter.entities.push({
    id: "reminder2",
    name: "Rats",
    initiative: 20,
    is_active: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
    win_initiative_tie: true,
  });

  const AddPlayer = () => {
    encounter.entities.push(playerForm);
    playerForm = PlayerEntityActions.EmptyPlayerEntity();
    addPlayerDialogOpen = false;
  };
</script>

{#snippet ConcentrationIcon()}
  <img
    src="/images/concentration.svg"
    alt="Concentration icon"
    class="ml-5 h-[25px] w-[25px]"
  />
{/snippet}

{#snippet ConditionsSection(combatEntity: CombatEntity)}
  <div class="flex w-full justify-between gap-5">
    <Title variant="muted">Conditions</Title>
    <Button onclick={(_) => CombatEntityActions.AddCondition(combatEntity)}>
      Add Condition
    </Button>
  </div>

  <!-- Exhaustion Level -->
  <div class="flex">
    <Container class="basis-2/3">
      <Label>Exhaustion Level</Label>
      <Select
        bind:value={combatEntity.exhaustion_level}
        placeholder="Select an exhaustion level"
        items={selectExhaustLevels}
      />
    </Container>
  </div>

  {#if combatEntity.conditions.length > 0}
    <hr />
  {/if}

  {#each combatEntity.conditions as condition, index (condition)}
    <div class="flex w-full gap-5">
      <!-- Condition -->
      <Container class="flex-1">
        <Label>Condition</Label>
        <Select
          bind:value={condition.condition}
          placeholder="Charmed"
          items={selectConditions}
        />
      </Container>

      <!-- Saving Throw Attribute -->
      <Container class="flex-1">
        <Label>Saving Throw Atribute</Label>
        <Select
          bind:value={condition.saving_throw_attribute}
          placeholder="Wisdom"
          items={selectAttributes}
        />
      </Container>

      <!-- Saving Throw DC -->
      <Container class="flex-1">
        <Label>Saving Throw DC</Label>
        <Input
          class="text-center"
          type="number"
          placeholder="13"
          bind:value={condition.saving_throw_dc}
        />
      </Container>
    </div>

    <div class="flex w-full gap-5">
      <!-- Source -->
      <Container class="min-w-0 flex-1">
        <Label>Source</Label>
        <Combobox
          placeholder="Lamia #1"
          bind:value={condition.source}
          items={[]}
        />
      </Container>

      <!-- Cause -->
      <Container class="flex-1">
        <Label>Cause</Label>
        <Input
          type="text"
          placeholder="Charm Person"
          bind:value={condition.cause}
        />
      </Container>

      <!-- Save Trigger -->
      <Container class="min-w-0 flex-1">
        <Label>Saving Throw Trigger</Label>
        <Select
          bind:value={condition.save_trigger}
          placeholder="Nothing"
          items={selectSaveTriggers}
        />
      </Container>
    </div>

    <div class="flex justify-end">
      <Button
        variant="destructive"
        onclick={(_) =>
          CombatEntityActions.RemoveCondition(combatEntity, condition)}
      >
        Remove
      </Button>
    </div>

    {#if index !== combatEntity.conditions.length - 1}
      <hr />
    {/if}
  {/each}
  <hr />
{/snippet}

{#snippet ActionsButton(entity: EncounterEntity)}
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      {#snippet child({ props })}
        <Button {...props} variant="ghost" size="icon" class="size-8">
          <Ellipsis />
        </Button>
      {/snippet}
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="w-56" align="center">
      <DropdownMenu.Group>
        <DropdownMenu.Item>Edit</DropdownMenu.Item>
        <DropdownMenu.Item
          onSelect={(e) => EncounterActions.RemoveEntity(encounter, entity, e)}
          closeOnSelect={true}
        >
          Remove
        </DropdownMenu.Item>
      </DropdownMenu.Group>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/snippet}

<!-- Creates a section for picking spells to add to a monster. -->
{#snippet MonsterPickSection(title: string, monsterType: MonsterType)}
  <Command.Group heading={title}>
    {#each monsters.filter((monster) => monster.monster_type === monsterType) as monster}
      <Command.Item class="flex justify-between" value={monster.name}>
        <span>{monster.challenge_rating}</span>
        <span class="text-muted-foreground">
          {DisplayMonsterType[monsterType]}
        </span>
      </Command.Item>
    {/each}
  </Command.Group>
{/snippet}

<!-- Health bar for monsters showing current, temp and max hit points. -->
{#snippet HealthBar(current: number, max: number, temp: number = 0)}
  {@const actualPercentage = Math.round((current / max) * 100)}
  {@const tempPercentage = Math.round((temp / max) * 100)}
  {@const healthColor =
    actualPercentage > 60
      ? "bg-green-500"
      : actualPercentage > 30
        ? "bg-yellow-500"
        : "bg-red-500"}

  <div class="relative h-6 w-full overflow-hidden rounded bg-gray-200">
    <!-- Regular health bar fill -->
    <div
      class="absolute inset-y-0 left-0 transition-all duration-300 {healthColor}"
      style:width="{actualPercentage}%"
    ></div>

    <!-- Temporary HP overlay (appears after regular HP) -->
    {#if temp > 0}
      <div
        class="absolute inset-y-0 bg-blue-400 opacity-80 transition-all duration-300"
        style:left="{actualPercentage}%"
        style:width="{tempPercentage}%"
      ></div>

      <!-- Striped pattern for temp HP (optional visual indicator) -->
      <div
        class="pointer-events-none absolute inset-y-0 opacity-30"
        style:left="{actualPercentage}%"
        style:width="{tempPercentage}%"
        style:background-image={`repeating-linear-gradient(45deg, transparent, transparent 5px, rgba(255,255,255,0.3) 5px, rgba(255,255,255,0.3) 10px)`}
      ></div>
    {/if}

    <!-- Text overlay -->
    <div class="relative flex h-full items-center justify-center">
      <span
        class="text-xs font-semibold text-white drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)]"
      >
        {current}{temp > 0 ? ` + ${temp}` : ""} / {max}
      </span>
    </div>
  </div>
{/snippet}

<div class="mx-auto mt-5 w-[1000px] space-y-5">
  <!-- Players Section -->
  <div class="flex justify-between">
    <Title variant="default" class="relative top-1">Players</Title>
    <Dialog
      title="Add new player"
      description="Add a new player to the encounter, click the button at the bottom when you are done."
      bind:open={addPlayerDialogOpen}
    >
      {#snippet trigger()}
        <Button variant="default">Add Player</Button>
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container class="flex-3">
            <Label>Name</Label>
            <Input
              type="text"
              placeholder="Player 1"
              bind:value={playerForm.name}
            />
          </Container>

          <!-- Initiative -->
          <Container class="flex-1">
            <Label>Initiative</Label>
            <Input
              class="text-center"
              type="number"
              placeholder="16"
              bind:value={playerForm.initiative}
            />
          </Container>

          <!-- Concentration -->
          <div class="flex flex-col justify-end">
            <Toggle bind:checked={playerForm.concentration}>
              Concentration
            </Toggle>
          </div>
        </div>

        <!-- Conditions -->
        {@render ConditionsSection(playerForm)}
      {/snippet}

      {#snippet footer()}
        <Button variant="default" onclick={(_) => AddPlayer()}>
          Add Player
        </Button>
      {/snippet}
    </Dialog>
  </div>
  <section class="mt-5 rounded-lg border">
    <!-- Header -->
    <div
      class="mt-5 mb-5 grid grid-cols-[2fr_1fr_6fr_1fr] px-6 py-1 text-xs font-medium tracking-wider uppercase"
    >
      <div>Name</div>
      <div class="text-center">Initiative</div>
      <div class="text-center">Details</div>
      <div class="text-center">Actions</div>
    </div>
    <hr />

    <!-- Player Rows -->
    {#each playerEntities as player, index (player.id!)}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_6fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">
          {player.name}
          {#if player.concentration}
            {@render ConcentrationIcon()}
          {/if}
        </div>
        <div class="flex items-center justify-center text-center text-sm">
          {player.initiative}
        </div>
        <div class="flex items-center text-sm font-medium">details</div>
        <div class="flex items-center justify-center text-sm">
          {@render ActionsButton(player)}
        </div>
      </div>

      {#if index !== playerEntities.length - 1}
        <hr />
      {/if}
    {/each}
  </section>

  <!-- Monsters Section -->
  <Title variant="default">Monsters</Title>
  <section class="mt-5 rounded-lg border">
    <!-- Header -->
    <div
      class="mt-5 mb-5 grid grid-cols-[2fr_1fr_3fr_3fr_1fr] px-6 py-1 text-xs font-medium tracking-wider uppercase"
    >
      <div>Name</div>
      <div class="text-center">Initiative</div>
      <div class="text-center">Details</div>
      <div class="text-center">HP</div>
      <div class="text-center">Actions</div>
    </div>
    <hr />

    <!-- Monster Rows -->
    {#each monsterEntities as monster, index (monster.id!)}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_3fr_3fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">
          {monster.name}
          {#if monster.concentration}
            {@render ConcentrationIcon()}
          {/if}
        </div>
        <div class="flex items-center justify-center text-center text-sm">
          {monster.initiative}
        </div>
        <div class="flex items-center text-sm font-medium">details</div>
        <div class="flex items-center text-sm font-medium">
          {@render HealthBar(
            monster.current_hp!,
            monster.max_hp!,
            monster.temporary_hp,
          )}
        </div>
        <div class="flex items-center justify-center text-sm">
          {@render ActionsButton(monster)}
        </div>
      </div>

      {#if index !== monsterEntities.length - 1}
        <hr />
      {/if}
    {/each}
  </section>

  <!-- Reminders Section -->
  <Title variant="default">Reminders</Title>
  <section class="mt-5 rounded-lg border">
    <!-- Header -->
    <div
      class="mt-5 mb-5 grid grid-cols-[2fr_1fr_1fr_5fr_1fr] px-6 py-1 text-xs font-medium tracking-wider uppercase"
    >
      <div>Name</div>
      <div class="text-center">Initiative</div>
      <div class="text-center">Wins Tie</div>
      <div class="text-center">Description</div>
      <div class="text-center">Actions</div>
    </div>
    <hr />

    <!-- Reminders Rows -->
    {#each reminderEntities as reminder, index (reminder.id!)}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_1fr_5fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">{reminder.name}</div>
        <div class="flex items-center justify-center text-center text-sm">
          {reminder.initiative}
        </div>
        <div class="flex items-center justify-center text-center text-sm">
          {reminder.win_initiative_tie ? "Yes" : "No"}
        </div>
        <div class="flex items-center text-sm font-medium">
          {reminder.description}
        </div>
        <div class="flex items-center justify-center text-sm">
          {@render ActionsButton(reminder)}
        </div>
      </div>

      {#if index !== reminderEntities.length - 1}
        <hr />
      {/if}
    {/each}
  </section>
</div>

<div class="mx-auto mt-5 grid w-[1000px] grid-cols-3">
  <!-- Input Section -->
  <!-- <div class="col-span-2"> -->
  <!--   Input Section -->
  <!---->
  <!--   <Command.Root class="col-span-8 w-full rounded-lg border"> -->
  <!--     <Command.Input placeholder="Search for a monster" /> -->
  <!--     <Command.List class="max-h-[800px]"> -->
  <!--       <Command.Empty>No results found.</Command.Empty> -->
  <!--       {#each MonsterTypes as monsterType, index} -->
  <!--         {@render MonsterPickSection( -->
  <!--           DisplayMonsterType[monsterType], -->
  <!--           monsterType, -->
  <!--         )} -->
  <!---->
  <!--         {#if index !== MonsterTypes.length - 1} -->
  <!--           <Command.Separator /> -->
  <!--         {/if} -->
  <!--       {/each} -->
  <!--     </Command.List> -->
  <!--   </Command.Root> -->
  <!-- </div> -->

  <!-- View Section -->
  <!-- <div class="col-span-1">View Section</div> -->
</div>
