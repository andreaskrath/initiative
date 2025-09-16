<script lang="ts">
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import Dices from "@lucide/svelte/icons/dices";
  import {
    Attributes,
    CombatEntityActions,
    ConditionsNoExhaustion,
    DisplayAttribute,
    DisplayCondition,
    DisplayExhaustionLevel,
    DisplayMonsterType,
    DisplayReminderType,
    DisplayTrigger,
    EncounterActions,
    ExhaustionLevel,
    ExhaustionLevels,
    MonsterActions,
    MonsterType,
    MonsterTypes,
    PlayerEntityActions,
    ReminderActions,
    ReminderType,
    ReminderTypes,
    RoundTriggers,
    SaveTriggers,
    Trigger,
    TurnTriggers,
    type CombatCondition,
    type CombatEntity,
    type EncounterEntity,
    type InitiativeReminder,
    type Monster,
    type MonsterEntity,
    type PlayerEntity,
    type RoundReminder,
    type TurnReminder,
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
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { ToLabelValueWith } from "$utils/factories";
  import { CreateFieldErrors, type FieldErrors } from "$utils/error";
  import Combobox from "$components/Combobox.svelte";
  import { ValidatePlayerEntity } from "$encounter/validate";
  import ConcentrationIcon from "$lib/shared/components/ConcentrationIcon.svelte";
  import CircleX from "@lucide/svelte/icons/circle-x";

  let addPlayerDialogOpen = $state(false);
  let addMonsterDialogOpen = $state(false);
  let addReminderDialogOpen = $state(false);
  let playerFormErrors: FieldErrors | null = $state(null);

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
  const selectSaveTriggers = ToLabelValueWith(SaveTriggers, DisplayTrigger);
  const selectExhaustLevels = ToLabelValueWith(
    ExhaustionLevels,
    DisplayExhaustionLevel,
  );
  const selectReminderTypes = ToLabelValueWith(
    ReminderTypes,
    DisplayReminderType,
  );
  const selectTurnReminderTriggers = ToLabelValueWith(
    TurnTriggers,
    DisplayTrigger,
  );
  const selectRoundReminderTriggers = ToLabelValueWith(
    RoundTriggers,
    DisplayTrigger,
  );
  let selectTurnReminderTargets: { label: string; value: string }[] = $derived(
    encounter.entities
      .filter((entity) => entity.type === "player" || entity.type === "monster")
      .map((entity) => ({
        label: entity.name!,
        value: entity.id!,
      })),
  );

  let playerForm: PlayerEntity = $state(
    PlayerEntityActions.EmptyPlayerEntity(),
  );

  let reminderName: string | undefined = $state(undefined);
  let reminderType: "initiative" | "turn" | "round" | undefined =
    $state(undefined);
  let initiativeReminder: InitiativeReminder = $state(
    ReminderActions.EmptyInitiativeReminder(),
  );
  let turnReminder: TurnReminder = $state(ReminderActions.EmptyTurnReminder());
  let roundReminder: RoundReminder = $state(
    ReminderActions.EmptyRoundReminder(),
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
    is_active: false,
    type: "reminder",
    description:
      "The house is falling apart and two random squares on the battle map become holes that you can fall through.",
    reminder_type: "initiative",
    initiative: 15,
  });
  encounter.entities.push({
    id: "reminder11",
    name: "House I",
    is_active: false,
    type: "reminder",
    description:
      "The house is falling apart and two random squares on the battle map become holes that you can fall through.",
    reminder_type: "initiative",
    initiative: 14,
  });
  encounter.entities.push({
    id: "reminder2",
    name: "Rats",
    is_active: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
    reminder_type: "turn",
    trigger: Trigger.StartOfTurn,
    targets: [],
  });
  encounter.entities.push({
    id: "reminder22",
    name: "Rats I",
    is_active: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
    reminder_type: "turn",
    trigger: Trigger.EndOfTurn,
    targets: [],
  });
  encounter.entities.push({
    id: "reminder3",
    name: "Cats I",
    is_active: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
    reminder_type: "round",
    trigger: Trigger.StartOfRound,
  });
  encounter.entities.push({
    id: "reminder33",
    name: "Cats II",
    is_active: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
    reminder_type: "round",
    trigger: Trigger.EndOfRound,
  });

  const AddPlayer = async () => {
    const result = await ValidatePlayerEntity(playerForm);

    if (result) {
      playerFormErrors = CreateFieldErrors(result);
      return;
    }

    encounter.entities.push(playerForm);
    playerForm = PlayerEntityActions.EmptyPlayerEntity();
    addPlayerDialogOpen = false;
    playerFormErrors = null;
  };

  const handleRollInitiative = () => {
    let roll = Math.round(Math.random() * 20);

    if (roll === 0) {
      roll = 1;
    }

    initiativeReminder.initiative = roll;
  };
</script>

{#snippet ConditionsSection(
  combatEntity: CombatEntity,
  errors: FieldErrors | null,
)}
  <div class="flex w-full justify-between gap-5">
    <Title variant="muted">Conditions</Title>
    <Button onclick={(_) => CombatEntityActions.AddCondition(combatEntity)}>
      Add Condition
    </Button>
  </div>

  <!-- Exhaustion Level -->
  <div class="flex">
    <Container class="basis-2/3">
      <Label required>Exhaustion Level</Label>
      <Select
        bind:value={combatEntity.exhaustion_level}
        placeholder="Select an exhaustion level"
        items={selectExhaustLevels}
        error={errors?.get("exhaustion_level")}
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
        <Label required>Condition</Label>
        <Select
          bind:value={condition.condition}
          placeholder="Charmed"
          items={selectConditions}
          error={errors?.get(`conditions.${index}.condition`)}
        />
      </Container>

      <!-- Saving Throw Attribute -->
      <Container class="flex-1">
        <Label required>Saving Throw Atribute</Label>
        <Select
          bind:value={condition.saving_throw_attribute}
          placeholder="Wisdom"
          items={selectAttributes}
          error={errors?.get(`conditions.${index}.saving_throw_attribute`)}
        />
      </Container>

      <!-- Saving Throw DC -->
      <Container class="flex-1">
        <Label required>Saving Throw DC</Label>
        <Input
          class="text-center"
          type="number"
          placeholder="13"
          bind:value={condition.saving_throw_dc}
          error={errors?.get(`conditions.${index}.saving_throw_dc`)}
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
        <Label required>Saving Throw Trigger</Label>
        <Select
          bind:value={condition.save_trigger}
          placeholder="Nothing"
          items={selectSaveTriggers}
          error={errors?.get(`conditions.${index}.save_trigger`)}
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
    <DropdownMenu.Content class="w-fit" align="center">
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
      title="Add a new player"
      description="Add a new player to the encounter, click the button at the bottom when you are done."
      bind:open={addPlayerDialogOpen}
    >
      {#snippet trigger()}
        <Button
          variant="ghost"
          class="text-green-500 hover:text-green-300 dark:text-green-300 dark:hover:text-green-500"
        >
          <CirclePlus />
        </Button>
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container class="flex-3">
            <Label required>Name</Label>
            <Input
              type="text"
              placeholder="Player 1"
              error={playerFormErrors?.get("name")}
              bind:value={playerForm.name}
            />
          </Container>

          <!-- Initiative -->
          <Container class="flex-1">
            <Label required>Initiative</Label>
            <Input
              class="text-center"
              type="number"
              placeholder="16"
              error={playerFormErrors?.get("initiative")}
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
        {@render ConditionsSection(playerForm, playerFormErrors)}
      {/snippet}

      {#snippet footer()}
        <div class="flex w-full flex-col gap-2">
          <div class="flex justify-end">
            <Button
              variant="default"
              class="w-fit"
              onclick={(_) => AddPlayer()}
            >
              Add Player
            </Button>
          </div>
        </div>
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
            <ConcentrationIcon />
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
  <div class="flex justify-between">
    <Title variant="default" class="relative top-1">Monsters</Title>
    <Dialog
      title="Add a new monster"
      description="Add a new monster to the encounter, click the button at the bottom when you are done."
      bind:open={addMonsterDialogOpen}
    >
      {#snippet trigger()}
        <Button
          variant="ghost"
          class="text-green-500 hover:text-green-300 dark:text-green-300 dark:hover:text-green-500"
        >
          <CirclePlus />
        </Button>
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container class="flex-3">
            <Label required>Name</Label>
            <Input
              type="text"
              placeholder="Player 1"
              error={playerFormErrors?.get("name")}
              bind:value={playerForm.name}
            />
          </Container>

          <!-- Initiative -->
          <Container class="flex-1">
            <Label required>Initiative</Label>
            <Input
              class="text-center"
              type="number"
              placeholder="16"
              error={playerFormErrors?.get("initiative")}
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
        {@render ConditionsSection(playerForm, playerFormErrors)}
      {/snippet}

      {#snippet footer()}
        <div class="flex w-full flex-col gap-2">
          <div class="flex justify-end">
            <Button
              variant="default"
              class="w-fit"
              onclick={(_) => AddPlayer()}
            >
              Add Player
            </Button>
          </div>
        </div>
      {/snippet}
    </Dialog>
  </div>
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
            <ConcentrationIcon />
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
  <div class="flex justify-between">
    <Title variant="default" class="relative top-1">Reminders</Title>
    <Dialog
      title="Add a new reminder"
      description="Add a new reminder to the encounter, click the button at the bottom when you are done."
      bind:open={addReminderDialogOpen}
    >
      {#snippet trigger()}
        <Button
          variant="ghost"
          class="text-green-500 hover:text-green-300 dark:text-green-300 dark:hover:text-green-500"
        >
          <CirclePlus />
        </Button>
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container class="flex-3">
            <Label required>Name</Label>
            <Input
              type="text"
              placeholder="Reminder 1"
              error={playerFormErrors?.get("name")}
              bind:value={reminderName}
            />
          </Container>

          <Container class="flex-3">
            <Label required>Type</Label>
            <Select
              bind:value={reminderType}
              placeholder="Select a type of reminder"
              items={selectReminderTypes}
            />
          </Container>
        </div>

        {#if reminderType === ReminderType.Initiative}
          <div class="flex w-full gap-5">
            <!-- Initiative -->
            <Container>
              <Label required>Initiative</Label>
              <div class="flex w-1/2 gap-2">
                <Input
                  type="number"
                  placeholder="15"
                  error={playerFormErrors?.get("name")}
                  bind:value={initiativeReminder.initiative}
                  class="text-center"
                />
                <Button onclick={(_) => handleRollInitiative()} class="mr-2">
                  <Dices />
                </Button>
              </div>
            </Container>
          </div>
        {:else if reminderType === ReminderType.Turn}
          <div class="flex w-full gap-5">
            <Container class="flex-1">
              <Label required>Trigger</Label>
              <Select
                bind:value={turnReminder.trigger}
                placeholder="Select a trigger"
                items={selectTurnReminderTriggers}
              />
            </Container>
          </div>

          <div class="flex w-full gap-5">
            <!-- Pick Targets -->
            <Command.Root class="h-[300px] flex-1 rounded-lg border">
              <Command.Input placeholder="Search for a spell" />
              <Command.List>
                <Command.Empty>No results found.</Command.Empty>
                <!-- Player -->
                <Command.Group heading="Players">
                  {#each encounter.entities.filter((entity) => entity.type === "player") as player}
                    <Command.Item
                      class="flex justify-between"
                      value={player.name}
                      onclick={(_) => turnReminder.targets.push(player.id!)}
                      disabled={turnReminder.targets.includes(player.id!)}
                    >
                      <span>{player.name}</span>
                    </Command.Item>
                  {/each}
                </Command.Group>

                <Command.Separator />

                <!-- Monsters -->
                <Command.Group heading="Monsters">
                  {#each encounter.entities.filter((entity) => entity.type === "monster") as monster}
                    <Command.Item
                      class="flex justify-between"
                      value={monster.name}
                      onclick={(_) => turnReminder.targets.push(monster.id!)}
                      disabled={turnReminder.targets.includes(monster.id!)}
                    >
                      <span>{monster.name}</span>
                    </Command.Item>
                  {/each}
                </Command.Group>
              </Command.List>
            </Command.Root>

            <!-- List Targets -->
            <ScrollArea
              class="h-[300px] flex-1 rounded-md border"
              scrollbarYClasses="hidden"
            >
              {#each encounter.entities
                .filter((entity) => turnReminder.targets.includes(entity.id!))
                .sort((a, b) => a.name!.localeCompare(b.name!)) as entity}
                <div class="mx-2 my-3 flex justify-between text-sm">
                  <span>{entity.name!}</span>
                  <div class="flex gap-x-2">
                    <CircleX
                      class="text-red-300 hover:text-red-600"
                      size="18"
                      onclick={(_: MouseEvent) =>
                        ReminderActions.RemoveTarget(turnReminder, entity.id!)}
                    />
                  </div>
                </div>
              {/each}
            </ScrollArea>
          </div>
        {:else if reminderType === ReminderType.Round}
          <div class="flex w-full gap-5">
            <Container class="flex-3">
              <Label required>Trigger</Label>
              <Select
                bind:value={roundReminder.trigger}
                placeholder="Select a trigger"
                items={selectRoundReminderTriggers}
              />
            </Container>

            <!-- Duration -->
          </div>
        {:else}
          <div class="mt-5 flex w-full justify-center gap-5">
            Select a reminder type to load more options.
          </div>
        {/if}
      {/snippet}

      {#snippet footer()}
        <div class="flex w-full flex-col gap-2">
          <div class="flex justify-end">
            <Button
              variant="default"
              class="w-fit"
              onclick={(_) => AddPlayer()}
            >
              Add Reminder
            </Button>
          </div>
        </div>
      {/snippet}
    </Dialog>
  </div>
  <section class="mt-5 rounded-lg border">
    <!-- Header -->
    <div
      class="mt-5 mb-5 flex px-6 py-1 text-xs font-medium tracking-wider uppercase"
    >
      <div class="flex-2">Name</div>
      <div class="flex-1 text-center">Trigger</div>
      <div class="flex-1 text-center">Details</div>
      <div class="flex-5 text-center">Description</div>
      <div class="flex-1 text-center">Actions</div>
    </div>
    <hr />

    <!-- Reminders Rows -->
    {#each reminderEntities as reminder, index (reminder.id!)}
      <div class="flex min-h-[50px] px-6 py-2">
        <div class="flex flex-2 items-center text-sm font-medium">
          {reminder.name}
        </div>
        <div
          class="flex flex-1 items-center justify-center text-center text-sm"
        >
          {ReminderActions.GetTrigger(reminder)}
        </div>
        <div
          class="flex flex-1 items-center justify-center text-center text-sm"
        >
          {ReminderActions.GetDetails(reminder)}
        </div>
        <div class="flex flex-5 items-center text-sm font-medium">
          {reminder.description}
        </div>
        <div class="flex flex-1 items-center justify-center text-sm">
          {@render ActionsButton(reminder)}
        </div>
      </div>

      {#if index !== reminderEntities.length - 1}
        <hr />
      {/if}
    {/each}
  </section>
</div>

<!-- Create Spell Button -->
<div class="fixed inset-x-0 bottom-0 mx-auto flex w-[1000px] justify-end pb-10">
  <Button onclick={async (e: MouseEvent) => console.log("button 2")}>
    Create Encounter
  </Button>
</div>
