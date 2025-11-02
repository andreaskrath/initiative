<script lang="ts">
  import type { EncounterEntity, MonsterEntity, PlayerEntity, ReminderEntity, Monster, Spell } from "$types";
  import {
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
    DisplaySpellLevel,
    DisplayTrigger,
    ReminderActions,
    SpellLevel,
    type MagicSchool,
  } from "$types";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import { GetModifier } from "$utils/convert";
  import ConcentrationIcon from "$components/ConcentrationIcon.svelte";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";

  interface Props {
    entity: EncounterEntity;
    monsters: Record<string, Monster>;
  }

  let { entity, monsters }: Props = $props();

  const isMonster = $derived(entity.type === "monster");
  const isPlayer = $derived(entity.type === "player");
  const isReminder = $derived(entity.type === "reminder");

  const monsterEntity = $derived(isMonster ? (entity as MonsterEntity) : null);
  const playerEntity = $derived(isPlayer ? (entity as PlayerEntity) : null);
  const reminderEntity = $derived(isReminder ? (entity as ReminderEntity) : null);

  // Look up monster data from the monsters map
  const monster = $derived(
    monsterEntity?.monster_id ? monsters[monsterEntity.monster_id] : null
  );

  // Track which spells are expanded
  let expandedSpells = $state(new Set<string>());

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
      {#if isMonster && monster && monsterEntity}

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
            <span class="font-semibold">Species:</span>
            {monster.species ?? "—"}
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
            <span class="font-semibold">Proficiency:</span>
            +{monster.proficiency_bonus ?? 0}
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

        <!-- Monster Reminders (Native + Combat Added) -->
        {@const nativeReminders = monster.reminders ?? []}
        {@const combatReminders = monsterEntity.reminders ?? []}
        {@const allReminders = [...nativeReminders, ...combatReminders]}

        <div>
          <h3 class="mb-2 font-semibold">Active Reminders</h3>
          {#if allReminders.length > 0}
            <div class="space-y-2">
              {#each allReminders as reminder, index}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{reminder.name ?? "Unnamed Reminder"}</h4>
                  <div class="text-sm text-muted-foreground">
                    {#if reminder.trigger}
                      <div class="mb-1 font-medium text-primary">
                        {DisplayTrigger[reminder.trigger]}
                      </div>
                    {/if}
                    {#if reminder.description}
                      <div>{reminder.description}</div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
              No active reminders
            </div>
          {/if}
        </div>

        <Separator />

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
          <Separator />
        {/if}

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

        <!-- Skills -->
        {#if monster.skills && monster.skills.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Skills</h3>
            <div class="grid grid-cols-2 gap-2 text-sm">
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
            <div class="grid grid-cols-2 gap-2 text-sm">
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

        <!-- Melee Attack Actions -->
        {#if monster.melee_attack_actions && monster.melee_attack_actions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Melee Attacks</h3>
            <div class="space-y-2">
              {#each monster.melee_attack_actions as attack}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{attack.name ?? "Unnamed Attack"}</h4>
                  <div class="text-sm text-muted-foreground space-y-1">
                    {#if attack.hit_bonus !== undefined}
                      <div>
                        <span class="font-medium">To Hit:</span>
                        {attack.hit_bonus >= 0 ? '+' : ''}{attack.hit_bonus}
                      </div>
                    {/if}
                    {#if attack.reach !== undefined}
                      <div>
                        <span class="font-medium">Reach:</span> {attack.reach} ft.
                      </div>
                    {/if}
                    {#if attack.one_handed_attack}
                      <div>
                        <span class="font-medium">Damage (1H):</span> {attack.one_handed_attack}
                        {#if attack.damage_type}
                          <span class="ml-1">({DisplayDamageType[attack.damage_type]})</span>
                        {/if}
                      </div>
                    {/if}
                    {#if attack.two_handed_attack}
                      <div>
                        <span class="font-medium">Damage (2H):</span> {attack.two_handed_attack}
                        {#if attack.damage_type}
                          <span class="ml-1">({DisplayDamageType[attack.damage_type]})</span>
                        {/if}
                      </div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Ranged Attack Actions -->
        {#if monster.ranged_attack_actions && monster.ranged_attack_actions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Ranged Attacks</h3>
            <div class="space-y-2">
              {#each monster.ranged_attack_actions as attack}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{attack.name ?? "Unnamed Attack"}</h4>
                  <div class="text-sm text-muted-foreground space-y-1">
                    {#if attack.hit_bonus !== undefined}
                      <div>
                        <span class="font-medium">To Hit:</span>
                        {attack.hit_bonus >= 0 ? '+' : ''}{attack.hit_bonus}
                      </div>
                    {/if}
                    {#if attack.normal_range !== undefined || attack.long_range !== undefined}
                      <div>
                        <span class="font-medium">Range:</span>
                        {attack.normal_range ?? 0}/{attack.long_range ?? 0} ft.
                      </div>
                    {/if}
                    {#if attack.attack}
                      <div>
                        <span class="font-medium">Damage:</span> {attack.attack}
                        {#if attack.damage_type}
                          <span class="ml-1">({DisplayDamageType[attack.damage_type]})</span>
                        {/if}
                      </div>
                    {/if}
                  </div>
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
                  {#if action.description}
                    <p class="text-sm text-muted-foreground">{action.description}</p>
                  {/if}
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
                  {#if action.description}
                    <p class="text-sm text-muted-foreground">{action.description}</p>
                  {/if}
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
                  {#if reaction.description}
                    <p class="text-sm text-muted-foreground">{reaction.description}</p>
                  {/if}
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

        <!-- Lair Actions -->
        {#if monster.lair_actions && monster.lair_actions.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Lair Actions</h3>
            <div class="space-y-2">
              {#each monster.lair_actions as action}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{action.name ?? "Unnamed Lair Action"}</h4>
                  {#if action.description}
                    <p class="text-sm text-muted-foreground">{action.description}</p>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Spellcasting -->
        {#if monster.spellcasting}
          <div>
            <h3 class="mb-2 font-semibold">Spellcasting</h3>

            <!-- Spellcasting Stats -->
            <div class="mb-3 grid grid-cols-2 gap-2 rounded-lg bg-muted p-3 text-sm">
              {#if monster.spellcasting.level !== undefined}
                <div>
                  <span class="font-medium">Caster Level:</span>
                  <span class="text-muted-foreground">{monster.spellcasting.level}</span>
                </div>
              {/if}
              {#if monster.spellcasting.attribute}
                <div>
                  <span class="font-medium">Spellcasting Ability:</span>
                  <span class="text-muted-foreground">{DisplayAttribute[monster.spellcasting.attribute]}</span>
                </div>
              {/if}
              {#if monster.spellcasting.dc !== undefined}
                <div>
                  <span class="font-medium">Spell Save DC:</span>
                  <span class="text-muted-foreground">{monster.spellcasting.dc}</span>
                </div>
              {/if}
              {#if monster.spellcasting.attack_bonus !== undefined}
                <div>
                  <span class="font-medium">Spell Attack:</span>
                  <span class="text-muted-foreground">
                    {monster.spellcasting.attack_bonus >= 0 ? '+' : ''}{monster.spellcasting.attack_bonus}
                  </span>
                </div>
              {/if}
            </div>

            <!-- Spell Slots -->
            {#if monster.spellcasting.spell_slots && monster.spellcasting.spell_slots.length > 0}
              <div class="mb-3">
                <h4 class="mb-2 text-sm font-medium">Spell Slots</h4>
                <div class="flex flex-wrap gap-2">
                  {#each monster.spellcasting.spell_slots as slot}
                    <span class="rounded bg-muted px-2 py-1 text-xs">
                      {DisplaySpellLevel[slot.level]}: {slot.slots}
                    </span>
                  {/each}
                </div>
              </div>
            {/if}

            <!-- Spells by Level -->
            {#if monster.spellcasting.spells && monster.spellcasting.spells.length > 0}
              <div class="space-y-3">
                {#each Array.from(spellsByLevel()).sort((a, b) => {
                  const order = [SpellLevel.Cantrip, SpellLevel.First, SpellLevel.Second, SpellLevel.Third, SpellLevel.Fourth, SpellLevel.Fifth, SpellLevel.Sixth, SpellLevel.Seventh, SpellLevel.Eighth, SpellLevel.Ninth];
                  return order.indexOf(a[0]) - order.indexOf(b[0]);
                }) as [level, spells]}
                  {@const levelKey = level as SpellLevel}
                  <div>
                    <h4 class="mb-2 text-sm font-semibold">
                      {DisplaySpellLevel[levelKey]} Level
                      <span class="text-xs text-muted-foreground">({spells.length})</span>
                    </h4>
                    <div class="space-y-1">
                      {#each spells as spell}
                        {@const spellId = spell.id ?? spell.name ?? Math.random().toString()}
                        {@const isExpanded = expandedSpells.has(spellId)}
                        <div class="rounded-lg border bg-card">
                          <!-- Spell Header (clickable) -->
                          <button
                            class="flex w-full items-center gap-2 p-2 text-left hover:bg-muted/50"
                            onclick={() => toggleSpell(spellId)}
                          >
                            <div class="flex-shrink-0">
                              {#if isExpanded}
                                <ChevronDown class="h-4 w-4" />
                              {:else}
                                <ChevronRight class="h-4 w-4" />
                              {/if}
                            </div>
                            <div class="flex-1">
                              <span class="font-medium">{spell.name ?? "Unnamed Spell"}</span>
                              {#if spell.concentration}
                                <span class="ml-1 text-xs text-blue-500">(C)</span>
                              {/if}
                              {#if spell.ritual}
                                <span class="ml-1 text-xs text-purple-500">(R)</span>
                              {/if}
                            </div>
                            {#if spell.school}
                              {@const schoolKey = spell.school as MagicSchool}
                              <span class="text-xs text-muted-foreground">
                                {DisplayMagicSchool[schoolKey]}
                              </span>
                            {/if}
                          </button>

                          <!-- Spell Details (collapsible) -->
                          {#if isExpanded}
                            <div class="border-t p-3 text-sm">
                              <div class="space-y-2">
                                <!-- Basic Info -->
                                <div class="grid grid-cols-2 gap-2">
                                  {#if spell.casting_time}
                                    <div>
                                      <span class="font-medium">Casting Time:</span>
                                      <span class="text-muted-foreground">{spell.casting_time}</span>
                                    </div>
                                  {/if}
                                  {#if spell.range}
                                    <div>
                                      <span class="font-medium">Range:</span>
                                      <span class="text-muted-foreground">{spell.range}</span>
                                    </div>
                                  {/if}
                                  {#if spell.duration}
                                    <div>
                                      <span class="font-medium">Duration:</span>
                                      <span class="text-muted-foreground">{spell.duration}</span>
                                    </div>
                                  {/if}
                                  {#if spell.area}
                                    <div>
                                      <span class="font-medium">Area:</span>
                                      <span class="text-muted-foreground">
                                        {spell.area}{spell.shape ? ` (${spell.shape})` : ''}
                                      </span>
                                    </div>
                                  {/if}
                                </div>

                                <!-- Components -->
                                <div>
                                  <span class="font-medium">Components:</span>
                                  <span class="text-muted-foreground">
                                    {#if spell.verbal}V{/if}{#if spell.verbal && (spell.somatic || spell.material)}, {/if}
                                    {#if spell.somatic}S{/if}{#if spell.somatic && spell.material}, {/if}
                                    {#if spell.material}M ({spell.material}{spell.material_consumed ? ', consumed' : ''}){/if}
                                    {#if !spell.verbal && !spell.somatic && !spell.material}None{/if}
                                  </span>
                                </div>

                                <!-- Description -->
                                {#if spell.description}
                                  <div>
                                    <p class="text-muted-foreground">{spell.description}</p>
                                  </div>
                                {/if}

                                <!-- At Higher Levels -->
                                {#if spell.at_higher_levels}
                                  <div>
                                    <span class="font-medium">At Higher Levels:</span>
                                    <p class="text-muted-foreground">{spell.at_higher_levels}</p>
                                  </div>
                                {/if}
                              </div>
                            </div>
                          {/if}
                        </div>
                      {/each}
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
          <Separator />
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

        <!-- Player Reminders -->
        <div>
          <h3 class="mb-2 font-semibold">Active Reminders</h3>
          {#if playerEntity.reminders && playerEntity.reminders.length > 0}
            <div class="space-y-2">
              {#each playerEntity.reminders as reminder}
                <div class="rounded-lg bg-muted p-3">
                  <h4 class="mb-1 font-semibold">{reminder.name ?? "Unnamed Reminder"}</h4>
                  <div class="text-sm text-muted-foreground">
                    {#if reminder.trigger}
                      <div class="mb-1 font-medium text-primary">
                        {DisplayTrigger[reminder.trigger]}
                      </div>
                    {/if}
                    {#if reminder.description}
                      <div>{reminder.description}</div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="rounded-lg border border-dashed p-4 text-center text-sm text-muted-foreground">
              No active reminders
            </div>
          {/if}
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
