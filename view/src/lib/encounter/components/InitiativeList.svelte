<script lang="ts">
  import type { EncounterEntity, MonsterEntity, PlayerEntity, ReminderEntity } from "$types";
  import { DisplayCondition, ReminderActions } from "$types";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { cn } from "$utils/utils";
  import ConcentrationIcon from "$components/ConcentrationIcon.svelte";
  import EntityActionMenu from "$encounter/components/EntityActionMenu.svelte";
  import Swords from "@lucide/svelte/icons/swords";
  import User from "@lucide/svelte/icons/user";
  import Bell from "@lucide/svelte/icons/bell";

  interface Props {
    entities: EncounterEntity[];
    activeIndex: number;
    selectedIndex: number;
    onSelectEntity: (index: number) => void;
    onToggleConcentration: (index: number) => void;
    onManageConditions: (index: number) => void;
    onTakeDamage: (index: number) => void;
    onHeal: (index: number) => void;
  }

  let {
    entities,
    activeIndex,
    selectedIndex,
    onSelectEntity,
    onToggleConcentration,
    onManageConditions,
    onTakeDamage,
    onHeal,
  }: Props = $props();

  const getConditionColor = (condition: string): string => {
    const colors: Record<string, string> = {
      blinded: "bg-gray-500",
      charmed: "bg-pink-500",
      deafened: "bg-gray-400",
      frightened: "bg-purple-500",
      grappled: "bg-yellow-600",
      incapacitated: "bg-red-600",
      invisible: "bg-blue-400",
      paralyzed: "bg-red-500",
      petrified: "bg-stone-500",
      poisoned: "bg-green-600",
      prone: "bg-orange-500",
      restrained: "bg-yellow-500",
      stunned: "bg-yellow-400",
      unconscious: "bg-gray-700",
      exhaustion: "bg-blue-600",
    };
    return colors[condition] ?? "bg-gray-500";
  };

  const getHealthPercentage = (entity: MonsterEntity): number => {
    if (!entity.current_hp || !entity.max_hp) return 0;
    return (entity.current_hp / entity.max_hp) * 100;
  };

  const getHealthColor = (percentage: number): string => {
    if (percentage > 66) return "bg-green-500";
    if (percentage > 33) return "bg-yellow-500";
    return "bg-red-500";
  };
</script>

<Container class="h-full">
  <Title variant="muted" class="mb-4">Initiative Order</Title>
  <ScrollArea class="h-[calc(100%-3rem)]">
    <div class="space-y-2 pr-4">
      {#each entities as entity, index}
        {@const isActive = index === activeIndex}
        {@const isSelected = index === selectedIndex}
        {@const isMonster = entity.type === "monster"}
        {@const isPlayer = entity.type === "player"}
        {@const isReminder = entity.type === "reminder"}

        <button
          class={cn(
            "w-full rounded-lg border-2 p-3 text-left transition-all hover:border-primary/50",
            isActive && isSelected
              ? "border-primary bg-primary/10"
              : isActive
                ? "border-primary/50 bg-primary/5"
                : isSelected
                  ? "border-primary/50 bg-accent"
                  : "border-border bg-card hover:bg-accent",
          )}
          onclick={() => onSelectEntity(index)}
        >
          <!-- Active Turn Indicator -->
          {#if isActive}
            <div class="mb-2 text-xs font-bold text-primary">› ACTIVE TURN</div>
          {/if}
          <!-- Header: Name and Initiative -->
          <div class="mb-2 flex items-center justify-between">
            <div class="flex flex-1 items-center gap-2">
              {#if isMonster}
                <Swords class="h-4 w-4 text-red-500" />
              {:else if isPlayer}
                <User class="h-4 w-4 text-blue-500" />
              {:else if isReminder}
                <Bell class="h-4 w-4 text-yellow-500" />
              {/if}
              <span class="font-semibold">{entity.name ?? "Unnamed"}</span>
              {#if entity.type !== "reminder"}
                <span
                  class="ml-2 rounded-full bg-primary/20 px-2 py-0.5 text-xs font-bold text-primary"
                >
                  {entity.initiative ?? 0}
                </span>
              {:else if entity.reminder_type === "initiative"}
                <span
                  class="ml-2 rounded-full bg-primary/20 px-2 py-0.5 text-xs font-bold text-primary"
                >
                  {entity.initiative ?? 0}
                </span>
              {/if}
            </div>

            <div class="flex items-center gap-2">
              <!-- Concentration indicator -->
              {#if entity.type !== "reminder" && entity.concentration}
                <ConcentrationIcon />
              {/if}

              <!-- Action Menu -->
              <EntityActionMenu
                {entity}
                onToggleConcentration={() => onToggleConcentration(index)}
                onManageConditions={() => onManageConditions(index)}
                onTakeDamage={() => onTakeDamage(index)}
                onHeal={() => onHeal(index)}
              />
            </div>
          </div>

          <!-- Health Bar (Monsters only) -->
          {#if isMonster}
            {@const healthPercent = getHealthPercentage(entity)}
            <div class="mb-2">
              <div class="mb-1 flex justify-between text-xs text-muted-foreground">
                <span>HP</span>
                <span>
                  {entity.current_hp ?? 0}
                  {#if entity.temporary_hp && entity.temporary_hp > 0}
                    <span class="text-blue-400">(+{entity.temporary_hp})</span>
                  {/if}
                  / {entity.max_hp ?? 0}
                </span>
              </div>
              <div class="h-2 overflow-hidden rounded-full bg-muted">
                <div
                  class={cn("h-full transition-all", getHealthColor(healthPercent))}
                  style="width: {healthPercent}%"
                ></div>
              </div>
            </div>
          {/if}

          <!-- Conditions as Badges -->
          {#if entity.type !== "reminder" && entity.conditions && entity.conditions.length > 0}
            <div class="flex flex-wrap gap-1">
              {#each entity.conditions as combatCondition}
                {#if combatCondition.condition}
                  <span
                    class={cn(
                      "rounded-full px-2 py-0.5 text-xs font-medium text-white",
                      getConditionColor(combatCondition.condition),
                    )}
                  >
                    {DisplayCondition[combatCondition.condition]}
                  </span>
                {/if}
              {/each}
            </div>
          {/if}

          <!-- Reminder Type -->
          {#if isReminder}
            <div class="text-xs text-muted-foreground">
              {ReminderActions.GetTrigger(entity)}
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </ScrollArea>
</Container>
