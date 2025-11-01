<script lang="ts">
  import type { EncounterEntity, MonsterEntity, PlayerEntity, ReminderEntity } from "$types";
  import {
    DisplayAlignment,
    DisplayCondition,
    DisplayDamageType,
    DisplayMonsterType,
    DisplayMovement,
    DisplaySight,
    DisplaySize,
    ReminderActions,
  } from "$types";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import { GetModifier } from "$utils/convert";
  import ConcentrationIcon from "$components/ConcentrationIcon.svelte";

  interface Props {
    entity: EncounterEntity;
  }

  let { entity }: Props = $props();

  const isMonster = $derived(entity.type === "monster");
  const isPlayer = $derived(entity.type === "player");
  const isReminder = $derived(entity.type === "reminder");

  const monsterEntity = $derived(isMonster ? (entity as MonsterEntity) : null);
  const playerEntity = $derived(isPlayer ? (entity as PlayerEntity) : null);
  const reminderEntity = $derived(isReminder ? (entity as ReminderEntity) : null);
</script>

<Container class="h-full">
  <ScrollArea class="h-full">
    <div class="space-y-4 pr-4">
      <!-- Header -->
      <div>
        <div class="flex items-center gap-2">
          <Title variant="default">{entity.name ?? "Unnamed"}</Title>
          {#if (isPlayer || isMonster) && ((playerEntity?.concentration ?? false) || (monsterEntity?.concentration ?? false))}
            <ConcentrationIcon />
          {/if}
        </div>
        {#if isReminder && reminderEntity}
          <p class="text-sm text-muted-foreground">
            {ReminderActions.GetTrigger(reminderEntity)}
          </p>
        {/if}
      </div>

      <Separator />

      <!-- Monster Details -->
      {#if isMonster && monsterEntity?.monster}
        {@const monster = monsterEntity.monster}

        <!-- Basic Info -->
        <div class="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span class="font-semibold">Type:</span>
            {monster.monster_type ? DisplayMonsterType[monster.monster_type] : "Unknown"}
          </div>
          <div>
            <span class="font-semibold">Size:</span>
            {monster.size ? DisplaySize[monster.size] : "Unknown"}
          </div>
          <div>
            <span class="font-semibold">Alignment:</span>
            {monster.alignment ? DisplayAlignment[monster.alignment] : "Unknown"}
          </div>
          <div>
            <span class="font-semibold">CR:</span>
            {monster.challenge_rating ?? "—"}
          </div>
          <div>
            <span class="font-semibold">AC:</span>
            {monster.armor_class ?? "—"}
          </div>
          <div>
            <span class="font-semibold">HP:</span>
            {monsterEntity.current_hp ?? 0} / {monsterEntity.max_hp ?? 0}
            {#if monsterEntity.temporary_hp && monsterEntity.temporary_hp > 0}
              <span class="text-blue-400">(+{monsterEntity.temporary_hp} temp)</span>
            {/if}
          </div>
        </div>

        <Separator />

        <!-- Speed -->
        {#if monster.speeds && monster.speeds.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Speed</h3>
            <div class="flex flex-wrap gap-2 text-sm">
              {#each monster.speeds as speed}
                <span class="rounded bg-muted px-2 py-1">
                  {speed.movement ? DisplayMovement[speed.movement] : "Walk"}: {speed.distance ?? 0} ft.
                </span>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Vision -->
        {#if monster.visions && monster.visions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Senses</h3>
            <div class="flex flex-wrap gap-2 text-sm">
              {#each monster.visions as vision}
                <span class="rounded bg-muted px-2 py-1">
                  {vision.sight ? DisplaySight[vision.sight] : "Darkvision"}: {vision.range ?? 0} ft.
                </span>
              {/each}
              {#if monster.passive_perception}
                <span class="rounded bg-muted px-2 py-1">
                  Passive Perception: {monster.passive_perception}
                </span>
              {/if}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Damage Resistances -->
        {#if monster.damage_resistances && monster.damage_resistances.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Damage Resistances</h3>
            <div class="flex flex-wrap gap-1">
              {#each monster.damage_resistances as damageType}
                <span class="rounded-full bg-blue-500/20 px-2 py-1 text-xs font-medium text-blue-700 dark:text-blue-300">
                  {DisplayDamageType[damageType]}
                </span>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Damage Immunities -->
        {#if monster.damage_immunities && monster.damage_immunities.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Damage Immunities</h3>
            <div class="flex flex-wrap gap-1">
              {#each monster.damage_immunities as damageType}
                <span class="rounded-full bg-purple-500/20 px-2 py-1 text-xs font-medium text-purple-700 dark:text-purple-300">
                  {DisplayDamageType[damageType]}
                </span>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Condition Immunities -->
        {#if monster.condition_immunities && monster.condition_immunities.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Condition Immunities</h3>
            <div class="flex flex-wrap gap-1">
              {#each monster.condition_immunities as condition}
                <span class="rounded-full bg-green-500/20 px-2 py-1 text-xs font-medium text-green-700 dark:text-green-300">
                  {DisplayCondition[condition]}
                </span>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Ability Scores -->
        <div>
          <h3 class="mb-2 font-semibold">Ability Scores</h3>
          <div class="grid grid-cols-6 gap-2 text-center text-sm">
            <div>
              <div class="font-semibold text-muted-foreground">STR</div>
              <div>{monster.strength ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.strength ?? 10)})
              </div>
            </div>
            <div>
              <div class="font-semibold text-muted-foreground">DEX</div>
              <div>{monster.dexterity ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.dexterity ?? 10)})
              </div>
            </div>
            <div>
              <div class="font-semibold text-muted-foreground">CON</div>
              <div>{monster.constitution ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.constitution ?? 10)})
              </div>
            </div>
            <div>
              <div class="font-semibold text-muted-foreground">INT</div>
              <div>{monster.intelligence ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.intelligence ?? 10)})
              </div>
            </div>
            <div>
              <div class="font-semibold text-muted-foreground">WIS</div>
              <div>{monster.wisdom ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.wisdom ?? 10)})
              </div>
            </div>
            <div>
              <div class="font-semibold text-muted-foreground">CHA</div>
              <div>{monster.charisma ?? 10}</div>
              <div class="text-xs text-muted-foreground">
                ({GetModifier(monster.charisma ?? 10)})
              </div>
            </div>
          </div>
        </div>

        <Separator />

        <!-- Traits -->
        {#if monster.traits && monster.traits.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Traits</h3>
            <div class="space-y-2">
              {#each monster.traits as trait}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{trait.name}</h4>
                  <p class="text-sm text-muted-foreground">{trait.description}</p>
                </div>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Actions -->
        {#if monster.regular_actions && monster.regular_actions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Actions</h3>
            <div class="space-y-2">
              {#each monster.regular_actions as action}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{action.name}</h4>
                  <p class="text-sm text-muted-foreground">{action.description}</p>
                </div>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Legendary Actions -->
        {#if monster.legendary_actions && monster.legendary_actions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Legendary Actions</h3>
            <div class="space-y-2">
              {#each monster.legendary_actions as action}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">
                    {action.name}
                    {#if action.cost && action.cost > 1}
                      <span class="text-xs text-muted-foreground">(Costs {action.cost} Actions)</span>
                    {/if}
                  </h4>
                  <p class="text-sm text-muted-foreground">{action.description}</p>
                </div>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Conditions on Monster -->
        {#if monsterEntity.conditions && monsterEntity.conditions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Active Conditions</h3>
            <div class="space-y-2">
              {#each monsterEntity.conditions as condition}
                {#if condition.condition}
                  <div class="rounded-lg bg-muted p-3">
                    <h4 class="mb-1 font-semibold">
                      {DisplayCondition[condition.condition]}
                    </h4>
                    <div class="text-sm text-muted-foreground">
                      {#if condition.source}
                        <div>Source: {condition.source}</div>
                      {/if}
                      {#if condition.cause}
                        <div>Cause: {condition.cause}</div>
                      {/if}
                      {#if condition.saving_throw_dc}
                        <div>Save DC: {condition.saving_throw_dc}</div>
                      {/if}
                    </div>
                  </div>
                {/if}
              {/each}
            </div>
          </div>
        {/if}

        <!-- Monster Reminders -->
        {#if monsterEntity.reminders && monsterEntity.reminders.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Reminders</h3>
            <div class="space-y-2">
              {#each monsterEntity.reminders as reminder}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{reminder.name}</h4>
                  <p class="text-sm text-muted-foreground">{reminder.description}</p>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      {/if}

      <!-- Player Details -->
      {#if isPlayer && playerEntity}
        <!-- Initiative -->
        <div>
          <h3 class="mb-2 font-semibold">Initiative</h3>
          <p class="text-2xl font-bold">{playerEntity.initiative ?? 0}</p>
        </div>

        <Separator />

        <!-- Conditions on Player -->
        {#if playerEntity.conditions && playerEntity.conditions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Active Conditions</h3>
            <div class="space-y-2">
              {#each playerEntity.conditions as condition}
                {#if condition.condition}
                  <div class="rounded-lg bg-muted p-3">
                    <h4 class="mb-1 font-semibold">
                      {DisplayCondition[condition.condition]}
                    </h4>
                    <div class="text-sm text-muted-foreground">
                      {#if condition.source}
                        <div>Source: {condition.source}</div>
                      {/if}
                      {#if condition.cause}
                        <div>Cause: {condition.cause}</div>
                      {/if}
                      {#if condition.saving_throw_dc}
                        <div>Save DC: {condition.saving_throw_dc}</div>
                      {/if}
                    </div>
                  </div>
                {/if}
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Player Reminders -->
        {#if playerEntity.reminders && playerEntity.reminders.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Reminders</h3>
            <div class="space-y-2">
              {#each playerEntity.reminders as reminder}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{reminder.name}</h4>
                  <p class="text-sm text-muted-foreground">{reminder.description}</p>
                </div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="rounded-lg bg-muted p-4 text-center text-muted-foreground">
            No reminders for this player
          </div>
        {/if}
      {/if}

      <!-- Reminder Details -->
      {#if isReminder && reminderEntity}
        <div>
          <h3 class="mb-2 font-semibold">Description</h3>
          <div class="rounded-lg bg-muted p-4">
            <p class="text-sm">{reminderEntity.description ?? "No description provided"}</p>
          </div>
        </div>

        {#if reminderEntity.reminder_type === "initiative"}
          <Separator />
          <div>
            <h3 class="mb-2 font-semibold">Initiative</h3>
            <p class="text-2xl font-bold">{reminderEntity.initiative ?? 0}</p>
          </div>
        {/if}
      {/if}
    </div>
  </ScrollArea>
</Container>
