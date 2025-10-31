<script lang="ts">
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import Dices from "@lucide/svelte/icons/dices";
  import { DiceRoll } from "@dice-roller/rpg-dice-roller";
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
    type ReminderEntity,
    type RoundReminder,
    MonsterEntityActions,
    RollStrategy,
    RollStrategies,
    DisplayRollStrategy,
  } from "$types";

  import { Button } from "$components/ui/button/index";
  import * as Command from "$components/ui/command/index";
  import * as DropdownMenu from "$components/ui/dropdown-menu/index";
  import Label from "$components/Label.svelte";
  import Input from "$components/Input.svelte";
  import Title from "$components/Title.svelte";
  import Container from "$components/Container.svelte";
  import { toast } from "svelte-sonner";
  import Toggle from "$components/Toggle.svelte";
  import Dialog from "$components/Dialog.svelte";
  import Select from "$components/Select.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { ToLabelValueWith } from "$utils/factories";
  import { CreateFieldErrors, type FieldErrors } from "$utils/error";
  import Combobox from "$components/Combobox.svelte";
  import * as ToggleGroup from "$components/ui/toggle-group/index";
  import {
    ValidatePlayerEntity,
    ValidateReminderEntity,
  } from "$encounter/validate";
  import ConcentrationIcon from "$components/ConcentrationIcon.svelte";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import { MonsterService } from "$monster/service";
  import { onMount } from "svelte";
  import { GetModifier } from "$utils/convert";
  import { EncounterService } from "$encounter/service";
  import { StatusCodes } from "http-status-codes";
  import { goto } from "@mateothegreat/svelte5-router";
  import { Roll } from "$utils/roll";
  import TextArea from "$components/TextArea.svelte";
  import AddButton from "$components/AddButton.svelte";

  let addPlayerDialogOpen = $state(false);
  let addMonsterDialogOpen = $state(false);
  let addReminderDialogOpen = $state(false);

  let encounterErrors: FieldErrors | null = $state(null);
  let playerFormErrors: FieldErrors | null = $state(null);
  let reminderFormErrors: FieldErrors | null = $state(null);
  let monsterFormErrors: FieldErrors | null = $state(null);

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
  const selectRoundReminderTriggers = ToLabelValueWith(
    RoundTriggers,
    DisplayTrigger,
  );
  let selectMonsters: { label: string; value: string }[] = $derived(
    monsters.map((monster) => ({
      label: monster.name!,
      value: monster.id!,
    })),
  );
  let playerForm: PlayerEntity = $state(
    PlayerEntityActions.EmptyPlayerEntity(),
  );

  let monsterId: string | undefined = $state(undefined);
  let selectedMonster = $derived(
    monsters.find((monster) => monster.id === monsterId),
  );
  let repeatMonster: number | undefined = $state(undefined);
  let monsterName: string | undefined = $state(undefined);
  let monsterRollStrategy: RollStrategy = $state(RollStrategy.Random);

  let reminderName: string | undefined = $state(undefined);
  let reminderType: "initiative" | "turn" | "round" | undefined =
    $state(undefined);
  let reminderDescription: string | undefined = $state(undefined);
  let initiativeReminder: InitiativeReminder = $state(
    ReminderActions.EmptyInitiativeReminder(),
  );
  let roundReminder: RoundReminder = $state(
    ReminderActions.EmptyRoundReminder(),
  );

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

  const AddReminder = async () => {
    let reminder;

    switch (reminderType) {
      case "initiative":
        reminder = initiativeReminder;
        break;
      case "round":
        reminder = roundReminder;
        break;
      default:
        return; // Should probably handle better in future.
    }

    reminder.name = reminderName;
    reminder.description = reminderDescription;

    const result = await ValidateReminderEntity(reminder);

    if (result) {
      reminderFormErrors = CreateFieldErrors(result);
      return;
    }

    encounter.entities.push(reminder);
    reminderName = undefined;
    reminderType = undefined;
    initiativeReminder = ReminderActions.EmptyInitiativeReminder();
    roundReminder = ReminderActions.EmptyRoundReminder();
    addReminderDialogOpen = false;
    reminderFormErrors = null;
  };

  const AddMonster = async () => {
    if (!monsterName || !selectedMonster) {
      return;
    }

    const addMonster = async (name: string) => {
      let monsterForm = MonsterEntityActions.EmptyMonsterEntity();
      monsterForm.name = name;
      const hp = Roll(
        selectedMonster!.rollable_hit_points!,
        monsterRollStrategy,
      );
      monsterForm.max_hp = hp;
      monsterForm.current_hp = hp;
      monsterForm.initiative = new DiceRoll(
        `1d20+${GetModifier(selectedMonster.dexterity!)}`,
      ).total;
      encounter.entities.push(monsterForm);
    };

    if (repeatMonster) {
      for (let i = 0; i < repeatMonster; i++) {
        let name = `${monsterName} #${i + 1}`;
        addMonster(name);
      }
    } else {
      addMonster(monsterName);
    }

    addMonsterDialogOpen = false;
    monsterName = undefined;
    monsterId = undefined;
    repeatMonster = undefined;
    monsterRollStrategy = RollStrategy.Random;
  };

  const CreateEncounter = async () => {
    const result = await EncounterService.Create(encounter);

    if (typeof result === "number") {
      encounterErrors = null;
      switch (result) {
        case StatusCodes.CREATED:
          toast.success("Successfully created encounter");
          goto("/encounters");
          break;
        case StatusCodes.INTERNAL_SERVER_ERROR:
          toast.success("Internal server error");
          break;
        default:
          toast.error("An unknown error occured");
      }
    } else {
      encounterErrors = result;
      toast.error("There is an issue with the defined defined encounter");
    }
  };

  onMount(async () => {
    monsters = await MonsterService.GetAll();
  });
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
  <!-- Encounter Name -->
  <div class="flex w-full gap-5">
    <!-- Name -->
    <Container>
      <Label required>Name</Label>
      <Input
        type="text"
        placeholder="Encounter 1"
        bind:value={encounter.name}
        error={encounterErrors?.get("name")}
      />
    </Container>
  </div>

  <!-- Players Section -->
  <div class="flex justify-between">
    <Title variant="default" class="relative top-1">Players</Title>
    <Dialog
      title="Add a new player"
      description="Add a new player to the encounter, click the button at the bottom when you are done."
      bind:open={addPlayerDialogOpen}
    >
      {#snippet trigger()}
        <AddButton />
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

    {#if playerEntities.length > 0}
      <hr />
    {/if}

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
        <AddButton />
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container class="flex-3">
            <Label required>Name</Label>
            <Input
              type="text"
              placeholder="Monster 1"
              bind:value={monsterName}
            />
          </Container>

          <!-- Monster -->
          <Container class="flex-3">
            <Label>Monster</Label>
            <Combobox
              placeholder="Select a monster"
              items={selectMonsters}
              bind:value={monsterId}
            />
          </Container>

          <!-- Repeat Monster -->
          <Container class="flex-1">
            <Label>Number</Label>
            <Input
              type="number"
              placeholder=""
              bind:value={repeatMonster}
              class="text-center"
            />
          </Container>
        </div>

        <!-- Roll Strategy for Monster HP -->
        <Container>
          <Label>HP Roll Strategy</Label>
          <ToggleGroup.Root
            type="single"
            variant="outline"
            class="w-full"
            bind:value={monsterRollStrategy}
          >
            {#each RollStrategies as rollStrategy}
              <ToggleGroup.Item value={rollStrategy}>
                {DisplayRollStrategy[rollStrategy]}
              </ToggleGroup.Item>
            {/each}
          </ToggleGroup.Root>
        </Container>
      {/snippet}

      {#snippet footer()}
        <div class="flex w-full flex-col gap-2">
          <div class="flex justify-end">
            <Button
              variant="default"
              class="w-fit"
              onclick={(_) => AddMonster()}
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

    {#if monsterEntities.length > 0}
      <hr />
    {/if}

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
        <AddButton />
      {/snippet}

      {#snippet content()}
        <div class="flex w-full gap-5">
          <!-- Name -->
          <Container>
            <Label required>Name</Label>
            <Input
              type="text"
              placeholder="Reminder 1"
              error={reminderFormErrors?.get("name")}
              bind:value={reminderName}
            />
          </Container>

          <Container>
            <Label required>Type</Label>
            <Select
              bind:value={reminderType}
              placeholder="Select a type of reminder"
              items={selectReminderTypes}
              error={reminderFormErrors?.get("reminder_type")}
            />
          </Container>
        </div>

        <Container>
          <Label required>Description</Label>
          <TextArea
            bind:value={reminderDescription}
            placeholder=""
            error={reminderFormErrors?.get("description")}
          />
        </Container>

        {#if reminderType === ReminderType.Initiative}
          <div class="flex w-full gap-5">
            <!-- Initiative -->
            <Container>
              <Label required>Initiative</Label>
              <div class="flex w-1/2 gap-2">
                <Input
                  type="number"
                  placeholder="15"
                  error={reminderFormErrors?.get("initiative")}
                  bind:value={initiativeReminder.initiative}
                  class="text-center"
                />
                <Button
                  onclick={(_) =>
                    (initiativeReminder.initiative = new DiceRoll(
                      "1d20",
                    ).total)}
                  class="mr-2"
                >
                  <Dices />
                </Button>
              </div>
            </Container>
          </div>
        {:else if reminderType === ReminderType.Round}
          <div class="flex w-full gap-5">
            <Container class="flex-3">
              <Label required>Trigger</Label>
              <Select
                bind:value={roundReminder.trigger}
                placeholder="Select a trigger"
                items={selectRoundReminderTriggers}
                error={reminderFormErrors?.get("trigger")}
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
              onclick={(_) => AddReminder()}
              disabled={!reminderType}
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

    {#if reminderEntities.length > 0}
      <hr />
    {/if}

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

<!-- Create Encounter Button -->
<div class="fixed inset-x-0 bottom-0 mx-auto flex w-[1000px] justify-end pb-10">
  <Button onclick={async (_) => CreateEncounter()}>Create Encounter</Button>
</div>
