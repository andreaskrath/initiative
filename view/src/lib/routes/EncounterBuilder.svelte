<script lang="ts">
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import {
    DisplayMonsterType,
    MonsterActions,
    MonsterType,
    MonsterTypes,
    type Monster,
  } from "$types";

  import { Button } from "$components/ui/button/index";
  import * as Command from "$components/ui/command/index";
  import { EncounterActions } from "$lib/types/Encounter";
  import Title from "$lib/shared/components/Title.svelte";

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

  encounter.entities.push({
    id: "something",
    name: "Roberto",
    initiative: 15,
    isActive: false,
    type: "player",
    player: {},
  });
  encounter.entities.push({
    id: "something-else",
    name: "Dixie",
    initiative: 4,
    isActive: false,
    type: "player",
    player: {},
  });

  encounter.entities.push({
    id: "something-else",
    name: "Goblin #1",
    initiative: 4,
    isActive: false,
    current_hp: 50,
    max_hp: 60,
    temporary_hp: 20,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
  });
  encounter.entities.push({
    id: "something-else",
    name: "Goblin #2",
    initiative: 5,
    isActive: false,
    current_hp: 60,
    max_hp: 160,
    temporary_hp: 0,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
  });
  encounter.entities.push({
    id: "something-else",
    name: "Goblin #3",
    initiative: 8,
    isActive: false,
    current_hp: 10,
    max_hp: 160,
    temporary_hp: 0,
    type: "monster",
    monster: MonsterActions.EmptyMonster(),
  });

  encounter.entities.push({
    id: "something-else",
    name: "House is falling apart",
    initiative: 20,
    isActive: false,
    type: "reminder",
    description:
      "The house is falling apart and two random squares on the battle map become holes that you can fall through.",
  });
  encounter.entities.push({
    id: "something-else",
    name: "Rats",
    initiative: 20,
    isActive: false,
    type: "reminder",
    description:
      "2 swarm of rats enter the house through the broken walls and floor.",
  });
</script>

<!-- Creates a section for picking spells to add to a monster. -->
{#snippet MonsterPickSection(title: string, monsterType: MonsterType)}
  <Command.Group heading={title}>
    {#each monsters.filter((monster) => monster.monster_type === monsterType) as monster}
      <Command.Item
        class="flex justify-between"
        value={monster.name}
        onclick={(e) => EncounterActions.AddMonster(encounter, monster, e)}
      >
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
  <Title variant="default">Players</Title>
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
    {#each playerEntities as player, index}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_6fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">{player.name}</div>
        <div class="flex items-center justify-center text-center text-sm">
          {player.initiative}
        </div>
        <div class="flex items-center text-sm font-medium">details</div>
        <div class="flex items-center justify-center text-sm">
          <Button variant="ghost" size="icon" class="size-8">
            <Ellipsis />
          </Button>
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
    {#each monsterEntities as monster, index}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_3fr_3fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">{monster.name}</div>
        <div class="flex items-center justify-center text-center text-sm">
          {monster.initiative}
        </div>
        <div class="flex items-center text-sm font-medium">details</div>
        <div class="flex items-center text-sm font-medium">
          {@render HealthBar(
            monster.current_hp,
            monster.max_hp,
            monster.temporary_hp,
          )}
        </div>
        <div class="flex items-center justify-center text-sm">
          <Button variant="ghost" size="icon" class="size-8">
            <Ellipsis />
          </Button>
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
      class="mt-5 mb-5 grid grid-cols-[2fr_1fr_6fr_1fr] px-6 py-1 text-xs font-medium tracking-wider uppercase"
    >
      <div>Name</div>
      <div class="text-center">Initiative</div>
      <div class="text-center">Description</div>
      <div class="text-center">Actions</div>
    </div>
    <hr />

    <!-- Reminders Rows -->
    {#each reminderEntities as reminder, index}
      <div class="grid min-h-[50px] grid-cols-[2fr_1fr_6fr_1fr] px-6 py-2">
        <div class="flex items-center text-sm font-medium">{reminder.name}</div>
        <div class="flex items-center justify-center text-center text-sm">
          {reminder.initiative}
        </div>
        <div class="flex items-center text-sm font-medium">
          {reminder.description}
        </div>
        <div class="flex items-center justify-center text-sm">
          <Button variant="ghost" size="icon" class="size-8">
            <Ellipsis />
          </Button>
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
