<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "@mateothegreat/svelte5-router";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import * as Tabs from "$components/ui/tabs/index";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import Edit from "@lucide/svelte/icons/edit";
  import Trash from "@lucide/svelte/icons/trash-2";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import type { Monster, Spell } from "$types";
  import {
    DisplayAlignment,
    DisplayAttribute,
    DisplayCondition,
    DisplayDamageType,
    DisplayLanguage,
    DisplayMonsterType,
    DisplayMovement,
    DisplaySight,
    DisplaySize,
    DisplaySkill,
    DisplaySpellLevel,
    DisplayTrigger,
    SpellLevel,
  } from "$types";
  import { GetModifier } from "$utils/convert";
  import { toast } from "svelte-sonner";

  let props = $props();

  let monster = $state<Monster | null>(null);
  let loading = $state(true);
  let expandedSpells = $state(new Set<string>());

  const monsterId = $derived(() => {
    const path = window.location.pathname;
    const match = path.match(/\/monsters\/view\/([^/]+)/);
    return match ? match[1] : null;
  });

  onMount(async () => {
    const id = monsterId();
    if (!id) {
      toast.error("Invalid monster ID");
      goto("/monsters");
      return;
    }

    try {
      const response = await fetch(`/api/monster/${id}`);
      if (response.ok) {
        monster = await response.json();
      } else {
        toast.error("Monster not found");
        goto("/monsters");
      }
    } catch (error) {
      console.error("Failed to load monster:", error);
      toast.error("Failed to load monster");
      goto("/monsters");
    } finally {
      loading = false;
    }
  });

  const deleteMonster = async () => {
    if (!monster?.id) return;

    if (!confirm(`Are you sure you want to delete "${monster.name}"?`)) {
      return;
    }

    try {
      const response = await fetch(`/api/monster/${monster.id}`, {
        method: "DELETE",
      });

      if (response.ok || response.status === 204) {
        toast.success("Monster deleted successfully");
        goto("/monsters");
      } else {
        toast.error("Failed to delete monster");
      }
    } catch (error) {
      console.error("Failed to delete monster:", error);
      toast.error("Failed to delete monster");
    }
  };

  const toggleSpell = (spellId: string) => {
    const newSet = new Set(expandedSpells);
    if (newSet.has(spellId)) {
      newSet.delete(spellId);
    } else {
      newSet.add(spellId);
    }
    expandedSpells = newSet;
  };

  // Group spells by level
  const spellsByLevel = $derived(() => {
    if (!monster?.spellcasting?.spells) return new Map();

    const grouped = new Map<SpellLevel, Spell[]>();
    for (const spell of monster.spellcasting.spells) {
      const level = spell.level ?? SpellLevel.Cantrip;
      if (!grouped.has(level)) {
        grouped.set(level, []);
      }
      grouped.get(level)!.push(spell);
    }
    return grouped;
  });
</script>

<Container class="mx-auto max-w-[1200px] py-4">
  <ScrollArea class="h-[calc(100vh-120px)]">
    <div class="space-y-4 pr-4">
      {#if loading}
        <div class="text-center text-muted-foreground">Loading monster...</div>
      {:else if monster}
        <!-- Header with Actions -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-4">
            <Button variant="ghost" size="icon" onclick={() => goto("/monsters")}>
              <ArrowLeft class="h-4 w-4" />
            </Button>
            <Title variant="default">{monster.name ?? "Unnamed Monster"}</Title>
          </div>
          <div class="flex gap-2">
            <Button variant="outline" onclick={deleteMonster}>
              <Trash class="mr-2 h-4 w-4" />
              Delete
            </Button>
          </div>
        </div>

        <Separator />

        <!-- Tabs for different sections -->
        <Tabs.Root value="info" class="w-full">
          <Tabs.List class="grid w-full grid-cols-3">
            <Tabs.Trigger value="info">Info</Tabs.Trigger>
            <Tabs.Trigger value="actions">Actions</Tabs.Trigger>
            <Tabs.Trigger value="spellcasting">Spellcasting</Tabs.Trigger>
          </Tabs.List>

          <!-- Info Tab -->
          <Tabs.Content value="info" class="space-y-4 mt-4">
            <!-- Basic Info -->
            <div class="grid grid-cols-2 gap-4 text-sm md:grid-cols-4">
              <div>
                <span class="font-semibold">Type:</span>
                {monster.monster_type ? DisplayMonsterType[monster.monster_type] : "Unknown"}
              </div>
              <div>
                <span class="font-semibold">Size:</span>
                {monster.size ? DisplaySize[monster.size] : "Unknown"}
              </div>
              <div>
                <span class="font-semibold">Species:</span>
                {monster.species ?? "—"}
              </div>
              <div>
                <span class="font-semibold">Alignment:</span>
                {monster.alignment ? DisplayAlignment[monster.alignment] : "Unknown"}
              </div>
              <div>
                <span class="font-semibold">CR:</span>
                {monster.challenge_rating ?? "—"} ({monster.xp ?? 0} XP)
              </div>
              <div>
                <span class="font-semibold">Proficiency:</span>
                +{monster.proficiency_bonus ?? 0}
              </div>
              <div>
                <span class="font-semibold">AC:</span>
                {monster.armor_class ?? "—"}
                {#if monster.armor_type}
                  ({monster.armor_type})
                {/if}
              </div>
              <div>
                <span class="font-semibold">HP:</span>
                {monster.hit_points ?? "—"}
                {#if monster.rollable_hit_points}
                  ({monster.rollable_hit_points})
                {/if}
              </div>
            </div>

            <Separator />

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

            <!-- Speeds -->
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

            <!-- Senses -->
            {#if (monster.visions && monster.visions.length > 0) || monster.passive_perception}
              <div>
                <h3 class="mb-2 font-semibold">Senses</h3>
                <div class="flex flex-wrap gap-2 text-sm">
                  {#if monster.visions}
                    {#each monster.visions as vision}
                      <span class="rounded bg-muted px-2 py-1">
                        {vision.sight ? DisplaySight[vision.sight] : "Darkvision"}: {vision.range ?? 0} ft.
                      </span>
                    {/each}
                  {/if}
                  {#if monster.passive_perception}
                    <span class="rounded bg-muted px-2 py-1">
                      Passive Perception: {monster.passive_perception}
                    </span>
                  {/if}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Languages -->
            {#if monster.languages && monster.languages.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Languages</h3>
                <div class="flex flex-wrap gap-1">
                  {#each monster.languages as language}
                    <span class="rounded-full bg-indigo-500/20 px-2 py-1 text-xs font-medium text-indigo-700 dark:text-indigo-300">
                      {DisplayLanguage[language]}
                    </span>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Resistances/Immunities -->
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

            <!-- Skills -->
            {#if monster.skills && monster.skills.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Skills</h3>
                <div class="grid grid-cols-2 gap-2 text-sm md:grid-cols-3">
                  {#each monster.skills as skillEntry}
                    <div>
                      <span class="font-medium">{DisplaySkill[skillEntry.skill]}:</span>
                      <span class="text-muted-foreground">
                        {skillEntry.modifier >= 0 ? '+' : ''}{skillEntry.modifier}
                      </span>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Saving Throws -->
            {#if monster.saving_throws && monster.saving_throws.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Saving Throws</h3>
                <div class="grid grid-cols-2 gap-2 text-sm md:grid-cols-3">
                  {#each monster.saving_throws as save}
                    <div>
                      <span class="font-medium">{DisplayAttribute[save.attribute]}:</span>
                      <span class="text-muted-foreground">
                        {save.modifier >= 0 ? '+' : ''}{save.modifier}
                      </span>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Traits -->
            {#if monster.traits && monster.traits.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Traits</h3>
                <div class="space-y-2">
                  {#each monster.traits as trait}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{trait.name ?? "Unnamed Trait"}</h4>
                      <p class="text-sm text-muted-foreground">{trait.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </Tabs.Content>

          <!-- Actions Tab -->
          <Tabs.Content value="actions" class="space-y-4 mt-4">
            <!-- Regular Actions -->
            {#if monster.regular_actions && monster.regular_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Actions</h3>
                <div class="space-y-2">
                  {#each monster.regular_actions as action}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{action.name ?? "Unnamed Action"}</h4>
                      <p class="text-sm text-muted-foreground">{action.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Melee Attacks -->
            {#if monster.melee_attack_actions && monster.melee_attack_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Melee Attacks</h3>
                <div class="space-y-2">
                  {#each monster.melee_attack_actions as attack}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{attack.name ?? "Unnamed Attack"}</h4>
                      <div class="text-sm text-muted-foreground space-y-1">
                        <div>Hit Bonus: {attack.hit_bonus ? (attack.hit_bonus >= 0 ? '+' : '') + attack.hit_bonus : '—'}</div>
                        <div>Reach: {attack.reach ?? '—'} ft.</div>
                        {#if attack.one_handed_attack}
                          <div>One-Handed: {attack.one_handed_attack}</div>
                        {/if}
                        {#if attack.two_handed_attack}
                          <div>Two-Handed: {attack.two_handed_attack}</div>
                        {/if}
                        {#if attack.damage_type}
                          <div>Damage Type: {DisplayDamageType[attack.damage_type]}</div>
                        {/if}
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Ranged Attacks -->
            {#if monster.ranged_attack_actions && monster.ranged_attack_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Ranged Attacks</h3>
                <div class="space-y-2">
                  {#each monster.ranged_attack_actions as attack}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{attack.name ?? "Unnamed Attack"}</h4>
                      <div class="text-sm text-muted-foreground space-y-1">
                        <div>Hit Bonus: {attack.hit_bonus ? (attack.hit_bonus >= 0 ? '+' : '') + attack.hit_bonus : '—'}</div>
                        <div>Range: {attack.normal_range ?? '—'} / {attack.long_range ?? '—'} ft.</div>
                        {#if attack.attack}
                          <div>Damage: {attack.attack}</div>
                        {/if}
                        {#if attack.damage_type}
                          <div>Damage Type: {DisplayDamageType[attack.damage_type]}</div>
                        {/if}
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Bonus Actions -->
            {#if monster.bonus_actions && monster.bonus_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Bonus Actions</h3>
                <div class="space-y-2">
                  {#each monster.bonus_actions as action}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{action.name ?? "Unnamed Action"}</h4>
                      <p class="text-sm text-muted-foreground">{action.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Reactions -->
            {#if monster.reactions && monster.reactions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Reactions</h3>
                <div class="space-y-2">
                  {#each monster.reactions as reaction}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{reaction.name ?? "Unnamed Reaction"}</h4>
                      <p class="text-sm text-muted-foreground">{reaction.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Recharge Actions -->
            {#if monster.recharge_actions && monster.recharge_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Recharge Actions</h3>
                <div class="space-y-2">
                  {#each monster.recharge_actions as action}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">
                        {action.name ?? "Unnamed Action"}
                        {#if action.recharge}
                          <span class="text-xs text-muted-foreground">(Recharge {action.recharge})</span>
                        {/if}
                      </h4>
                      <p class="text-sm text-muted-foreground">{action.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Legendary Actions -->
            {#if monster.legendary_actions && monster.legendary_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">
                  Legendary Actions
                  {#if monster.available_legendary_actions_per_turn}
                    <span class="text-xs text-muted-foreground">({monster.available_legendary_actions_per_turn} per turn)</span>
                  {/if}
                </h3>
                <div class="space-y-2">
                  {#each monster.legendary_actions as action}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">
                        {action.name ?? "Unnamed Action"}
                        {#if action.cost && action.cost > 1}
                          <span class="text-xs text-muted-foreground">(Costs {action.cost} Actions)</span>
                        {/if}
                      </h4>
                      <p class="text-sm text-muted-foreground">{action.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
              <Separator />
            {/if}

            <!-- Lair Actions -->
            {#if monster.lair_actions && monster.lair_actions.length > 0}
              <div>
                <h3 class="mb-2 font-semibold">Lair Actions</h3>
                <div class="space-y-2">
                  {#each monster.lair_actions as action}
                    <div class="rounded-lg bg-muted p-3">
                      <h4 class="mb-1 font-semibold">{action.name ?? "Unnamed Action"}</h4>
                      <p class="text-sm text-muted-foreground">{action.description ?? ""}</p>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}
          </Tabs.Content>

          <!-- Spellcasting Tab -->
          <Tabs.Content value="spellcasting" class="space-y-4 mt-4">
            {#if monster.spellcasting && monster.spellcasting.spells && monster.spellcasting.spells.length > 0}
              <!-- Spellcasting Info -->
              <div class="grid grid-cols-2 gap-4 text-sm md:grid-cols-4">
                {#if monster.spellcasting.level}
                  <div>
                    <span class="font-semibold">Caster Level:</span> {monster.spellcasting.level}
                  </div>
                {/if}
                {#if monster.spellcasting.attribute}
                  <div>
                    <span class="font-semibold">Casting Ability:</span>
                    {DisplayAttribute[monster.spellcasting.attribute]}
                  </div>
                {/if}
                {#if monster.spellcasting.dc}
                  <div>
                    <span class="font-semibold">Spell Save DC:</span> {monster.spellcasting.dc}
                  </div>
                {/if}
                {#if monster.spellcasting.attack_bonus}
                  <div>
                    <span class="font-semibold">Spell Attack:</span>
                    {monster.spellcasting.attack_bonus >= 0 ? '+' : ''}{monster.spellcasting.attack_bonus}
                  </div>
                {/if}
              </div>

              <Separator />

              <!-- Spell Slots -->
              {#if monster.spellcasting.spell_slots && monster.spellcasting.spell_slots.length > 0}
                <div>
                  <h3 class="mb-2 font-semibold">Spell Slots</h3>
                  <div class="flex flex-wrap gap-2 text-sm">
                    {#each monster.spellcasting.spell_slots as slot}
                      <span class="rounded bg-muted px-2 py-1">
                        {DisplaySpellLevel[slot.level]}: {slot.slots} slots
                      </span>
                    {/each}
                  </div>
                </div>
                <Separator />
              {/if}

              <!-- Spells by Level -->
              {#each Array.from(spellsByLevel()) as [level, spells]}
                {@const spellLevel = level as SpellLevel}
                <div>
                  <h3 class="mb-2 font-semibold">{DisplaySpellLevel[spellLevel]}</h3>
                  <div class="space-y-2">
                    {#each spells as spell}
                      <div class="rounded-lg border bg-card">
                        <button
                          class="flex w-full items-center justify-between p-3 text-left transition-colors hover:bg-accent"
                          onclick={() => toggleSpell(spell.id!)}
                        >
                          <div class="flex items-center gap-2">
                            <span class="font-semibold">{spell.name ?? "Unnamed Spell"}</span>
                            {#if spell.concentration}
                              <span class="text-xs text-blue-500">(C)</span>
                            {/if}
                            {#if spell.ritual}
                              <span class="text-xs text-purple-500">(R)</span>
                            {/if}
                          </div>
                          {#if expandedSpells.has(spell.id!)}
                            <ChevronDown class="h-4 w-4" />
                          {:else}
                            <ChevronRight class="h-4 w-4" />
                          {/if}
                        </button>
                        {#if expandedSpells.has(spell.id!)}
                          <div class="border-t p-3 text-sm space-y-2">
                            <div class="grid grid-cols-2 gap-2">
                              {#if spell.casting_time}
                                <div>
                                  <span class="font-semibold">Casting Time:</span> {spell.casting_time}
                                </div>
                              {/if}
                              {#if spell.range}
                                <div>
                                  <span class="font-semibold">Range:</span> {spell.range}
                                </div>
                              {/if}
                              {#if spell.duration}
                                <div>
                                  <span class="font-semibold">Duration:</span> {spell.duration}
                                </div>
                              {/if}
                              {#if spell.school}
                                <div>
                                  <span class="font-semibold">School:</span> {spell.school}
                                </div>
                              {/if}
                            </div>
                            {#if spell.description}
                              <div>
                                <span class="font-semibold">Description:</span>
                                <p class="mt-1 text-muted-foreground">{spell.description}</p>
                              </div>
                            {/if}
                            {#if spell.higher_levels}
                              <div>
                                <span class="font-semibold">At Higher Levels:</span>
                                <p class="mt-1 text-muted-foreground">{spell.higher_levels}</p>
                              </div>
                            {/if}
                          </div>
                        {/if}
                      </div>
                    {/each}
                  </div>
                </div>
                {#if level !== Array.from(spellsByLevel())[Array.from(spellsByLevel()).length - 1][0]}
                  <Separator />
                {/if}
              {/each}
            {:else}
              <div class="rounded-lg border border-dashed p-8 text-center text-muted-foreground">
                This monster has no spellcasting abilities.
              </div>
            {/if}
          </Tabs.Content>
        </Tabs.Root>
      {/if}
    </div>
  </ScrollArea>
</Container>
