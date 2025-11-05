<script lang="ts">
  import type { EncounterEntity, MonsterEntity, PlayerEntity } from "$types";
  import * as DropdownMenu from "$components/ui/dropdown-menu/index";
  import { Button } from "$components/ui/button/index";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import Heart from "@lucide/svelte/icons/heart";
  import Skull from "@lucide/svelte/icons/skull";
  import Sparkles from "@lucide/svelte/icons/sparkles";
  import Shield from "@lucide/svelte/icons/shield";
  import Eye from "@lucide/svelte/icons/eye";
  import Bell from "@lucide/svelte/icons/bell";
  import Hash from "@lucide/svelte/icons/hash";
  import Trash2 from "@lucide/svelte/icons/trash-2";

  interface Props {
    entity: EncounterEntity;
    onToggleConcentration?: () => void;
    onManageConditions?: () => void;
    onManageReminders?: () => void;
    onTakeDamage?: () => void;
    onHeal?: () => void;
    onEditInitiative?: () => void;
    onRemoveEntity?: () => void;
  }

  let {
    entity,
    onToggleConcentration,
    onManageConditions,
    onManageReminders,
    onTakeDamage,
    onHeal,
    onEditInitiative,
    onRemoveEntity,
  }: Props = $props();

  const isMonster = $derived(entity.type === "monster");
  const isPlayer = $derived(entity.type === "player");
  const isCombatEntity = $derived(isMonster || isPlayer);
</script>

{#if isCombatEntity}
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      <Button variant="ghost" size="icon" class="h-8 w-8" onclick={(e) => e.stopPropagation()}>
        <Ellipsis class="h-4 w-4" />
      </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content align="end">
      <DropdownMenu.Label>Actions</DropdownMenu.Label>
      <DropdownMenu.Separator />

      <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onEditInitiative?.(); }}>
        <Hash class="mr-2 h-4 w-4" />
        Edit Initiative
      </DropdownMenu.Item>

      {#if isMonster}
        <DropdownMenu.Separator />
        <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onTakeDamage?.(); }}>
          <Skull class="mr-2 h-4 w-4" />
          Take Damage
        </DropdownMenu.Item>
        <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onHeal?.(); }}>
          <Heart class="mr-2 h-4 w-4" />
          Heal
        </DropdownMenu.Item>
      {/if}

      <DropdownMenu.Separator />

      <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onToggleConcentration?.(); }}>
        <Sparkles class="mr-2 h-4 w-4" />
        Toggle Concentration
      </DropdownMenu.Item>
      <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onManageConditions?.(); }}>
        <Shield class="mr-2 h-4 w-4" />
        Manage Conditions
      </DropdownMenu.Item>
      <DropdownMenu.Item onclick={(e) => { e.stopPropagation(); onManageReminders?.(); }}>
        <Bell class="mr-2 h-4 w-4" />
        Manage Reminders
      </DropdownMenu.Item>

      <DropdownMenu.Separator />

      <DropdownMenu.Item
        onclick={(e) => { e.stopPropagation(); onRemoveEntity?.(); }}
        class="text-destructive focus:text-destructive"
      >
        <Trash2 class="mr-2 h-4 w-4" />
        Remove from Encounter
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/if}
