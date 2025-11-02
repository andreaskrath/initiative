<script lang="ts">
  import { onMount } from "svelte";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import {
    Condition,
    DisplayCondition,
    Conditions,
    ExhaustionLevel,
    MonsterActions,
    type Encounter,
    type EncounterEntity,
    type MonsterEntity,
    type PlayerEntity,
    type CombatCondition,
    type CreatureReminder,
    MonsterType,
    Size,
    Alignment,
    Attribute,
    DisplayAttribute,
    Attributes,
    Movement,
    Trigger,
    DamageType,
    DisplayDamageType,
    DamageTypes,
    Sight,
    PlayerEntityActions,
    MonsterEntityActions,
    ReminderActions,
    type InitiativeReminder,
    type RoundReminder,
    type Monster,
    ReminderType,
    DisplayReminderType,
    ReminderTypes,
    DisplayTrigger,
    RoundTriggers,
    TurnTriggers,
    RollStrategy,
    RollStrategies,
    DisplayRollStrategy,
  } from "$types";
  import { v4 as uuid } from "uuid";
  import InitiativeList from "$encounter/components/InitiativeList.svelte";
  import EntityDetailsPanel from "$encounter/components/EntityDetailsPanel.svelte";
  import * as Dialog from "$components/ui/dialog/index";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import Select from "$components/Select.svelte";
  import TextArea from "$components/TextArea.svelte";
  import Combobox from "$components/Combobox.svelte";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import * as Tabs from "$components/ui/tabs/index";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import X from "@lucide/svelte/icons/x";
  import Plus from "@lucide/svelte/icons/plus";
  import Dices from "@lucide/svelte/icons/dices";
  import { toast } from "svelte-sonner";
  import { MonsterService } from "$monster/service";
  import { GetModifier } from "$utils/convert";
  import { Roll } from "$utils/roll";
  import { DiceRoll } from "@dice-roller/rpg-dice-roller";
  import { ToLabelValueWith } from "$utils/factories";
  import { CompareEncounterEntities } from "$utils/sort";
  import { useAutoSave } from "$utils/autosave.svelte";
  import { EncounterActions } from "$types";
  import BattleMap from "$encounter/components/BattleMap.svelte";
  import Map from "@lucide/svelte/icons/map";

  // Extract encounter ID directly from URL path
  // URL format: /encounters/view/{id}
  const getEncounterIdFromUrl = (): string | null => {
    const pathParts = window.location.pathname.split("/").filter((p) => p);
    // pathParts should be: ['encounters', 'view', 'id']
    if (
      pathParts.length >= 3 &&
      pathParts[0] === "encounters" &&
      pathParts[1] === "view"
    ) {
      return pathParts[2];
    }
    return null;
  };

  const encounterId = getEncounterIdFromUrl();

  // Initialize with empty encounter, will load from backend if ID exists
  let encounter = $state<Encounter>(EncounterActions.EmptyEncounter());
  let loading = $state(true);

  // Old dummy encounter for reference (will be removed)
  /*
  let encounter = $state<Encounter>({
    id: uuid(),
    name: "Goblin Ambush",
    active: 0,
    entities: [
      // Player 1
      {
        id: uuid(),
        type: "player",
        name: "Thorin Ironshield",
        initiative: 18,
        concentration: false,
        exhaustion_level: ExhaustionLevel.None,
        conditions: [],
        reminders: [],
      } satisfies PlayerEntity,

      // Monster 1
      {
        id: uuid(),
        type: "monster",
        name: "Goblin Scout",
        initiative: 16,
        concentration: false,
        exhaustion_level: ExhaustionLevel.None,
        conditions: [
          { condition: Condition.Poisoned, source: "Thorin", cause: "Poison Spray" },
        ],
        reminders: [],
        current_hp: 5,
        max_hp: 7,
        temporary_hp: 0,
        monster: {
          id: uuid(),
          name: "Goblin",
          challenge_rating: 0.25,
          xp: 50,
          proficiency_bonus: 2,
          size: Size.Small,
          monster_type: MonsterType.Humanoid,
          alignment: Alignment.NeutralEvil,
          armor_class: 15,
          hit_points: 7,
          speeds: [{ movement: Movement.Normal, distance: 30 }],
          strength: 8,
          dexterity: 14,
          constitution: 10,
          intelligence: 10,
          wisdom: 8,
          charisma: 8,
          visions: [{ sight: Sight.Darkvision, range: 60 }],
          skills: [],
          saving_throws: [],
          damage_resistances: [],
          damage_immunities: [],
          condition_immunities: [],
          languages: [],
          passive_perception: 9,
          traits: [
            {
              name: "Nimble Escape",
              description:
                "The goblin can take the Disengage or Hide action as a bonus action on each of its turns.",
            },
          ],
          regular_actions: [
            {
              name: "Scimitar",
              description: "Melee Weapon Attack: +4 to hit, reach 5 ft., one target. Hit: 5 (1d6 + 2) slashing damage.",
            },
          ],
          melee_attack_actions: [],
          ranged_attack_actions: [],
          recharge_actions: [],
          bonus_actions: [],
          reactions: [],
          legendary_actions: [],
          lair_actions: [],
          reminders: [],
          spellcasting: undefined,
        },
      } satisfies MonsterEntity,

      // Reminder (Initiative)
      {
        id: uuid(),
        type: "reminder",
        name: "Lair Action",
        reminder_type: "initiative",
        initiative: 10,
        description: "The dragon uses its lair action to cause tremors in the cave.",
      },

      // Player 2
      {
        id: uuid(),
        type: "player",
        name: "Elara Moonwhisper",
        initiative: 14,
        concentration: true,
        exhaustion_level: ExhaustionLevel.None,
        conditions: [],
        reminders: [],
      } satisfies PlayerEntity,

      // Monster 2
      {
        id: uuid(),
        type: "monster",
        name: "Goblin Boss",
        initiative: 13,
        concentration: false,
        exhaustion_level: ExhaustionLevel.None,
        conditions: [
          { condition: Condition.Frightened, source: "Elara", cause: "Fear spell" },
          { condition: Condition.Prone },
        ],
        reminders: [],
        current_hp: 15,
        max_hp: 21,
        temporary_hp: 3,
        monster: {
          id: uuid(),
          name: "Goblin Boss",
          challenge_rating: 1,
          xp: 200,
          proficiency_bonus: 2,
          size: Size.Small,
          monster_type: MonsterType.Humanoid,
          alignment: Alignment.NeutralEvil,
          armor_class: 17,
          hit_points: 21,
          speeds: [{ movement: Movement.Normal, distance: 30 }],
          strength: 10,
          dexterity: 14,
          constitution: 10,
          intelligence: 10,
          wisdom: 8,
          charisma: 10,
          visions: [{ sight: Sight.Darkvision, range: 60 }],
          skills: [],
          saving_throws: [],
          damage_resistances: [DamageType.Poison],
          damage_immunities: [DamageType.Fire],
          condition_immunities: [Condition.Charmed],
          languages: [],
          passive_perception: 9,
          traits: [
            {
              name: "Redirect Attack",
              description:
                "When a creature the goblin can see targets it with an attack, the goblin chooses another goblin within 5 feet of it. The two goblins swap places, and the chosen goblin becomes the target instead.",
            },
          ],
          regular_actions: [
            {
              name: "Multiattack",
              description: "The goblin makes two scimitar attacks.",
            },
            {
              name: "Scimitar",
              description: "Melee Weapon Attack: +4 to hit, reach 5 ft., one target. Hit: 5 (1d6 + 2) slashing damage.",
            },
          ],
          melee_attack_actions: [],
          ranged_attack_actions: [],
          recharge_actions: [],
          bonus_actions: [],
          reactions: [],
          legendary_actions: [],
          lair_actions: [],
          reminders: [
            {
              id: uuid(),
              type: "reminder",
              name: "Goblin Regeneration",
              description: "The goblin boss regains 5 hit points at the start of its turn.",
              trigger: Trigger.StartOfTurn,
            },
          ],
          spellcasting: undefined,
        },
      } satisfies MonsterEntity,

      // Monster 3
      {
        id: uuid(),
        type: "monster",
        name: "Goblin Archer",
        initiative: 12,
        concentration: false,
        exhaustion_level: ExhaustionLevel.None,
        conditions: [],
        reminders: [],
        current_hp: 7,
        max_hp: 7,
        temporary_hp: 0,
        monster: {
          id: uuid(),
          name: "Goblin",
          challenge_rating: 0.25,
          xp: 50,
          proficiency_bonus: 2,
          size: Size.Small,
          monster_type: MonsterType.Humanoid,
          alignment: Alignment.NeutralEvil,
          armor_class: 15,
          hit_points: 7,
          speeds: [{ movement: Movement.Normal, distance: 30 }],
          strength: 8,
          dexterity: 14,
          constitution: 10,
          intelligence: 10,
          wisdom: 8,
          charisma: 8,
          visions: [{ sight: Sight.Darkvision, range: 60 }],
          skills: [],
          saving_throws: [],
          damage_resistances: [DamageType.Cold],
          damage_immunities: [],
          condition_immunities: [],
          languages: [],
          passive_perception: 9,
          traits: [],
          regular_actions: [
            {
              name: "Shortbow",
              description: "Ranged Weapon Attack: +4 to hit, range 80/320 ft., one target. Hit: 5 (1d6 + 2) piercing damage.",
            },
          ],
          melee_attack_actions: [],
          ranged_attack_actions: [],
          recharge_actions: [],
          bonus_actions: [],
          reactions: [],
          legendary_actions: [],
          lair_actions: [],
          reminders: [],
          spellcasting: undefined,
        },
      } satisfies MonsterEntity,
    ],
  });
  */

  // Sort entities by initiative (descending)
  let sortedEntities = $derived(
    [...encounter.entities].sort((a, b) => {
      const aInit =
        a.type === "reminder" && a.reminder_type === "round"
          ? -1
          : (a.initiative ?? 0);
      const bInit =
        b.type === "reminder" && b.reminder_type === "round"
          ? -1
          : (b.initiative ?? 0);
      return bInit - aInit;
    }),
  );

  // Round counter
  let currentRound = $state(1);

  // Selected entity for viewing (separate from active turn)
  let selectedEntityIndex = $state(0);

  // Dialog state
  let damageDialogOpen = $state(false);
  let healDialogOpen = $state(false);
  let conditionDialogOpen = $state(false);
  let reminderManagementDialogOpen = $state(false);
  let addEntityDialogOpen = $state(false);
  let battleMapDialogOpen = $state(false);
  let damageAmount = $state<number | undefined>(undefined);
  let damageType = $state<string | undefined>(undefined);
  let healAmount = $state<number | undefined>(undefined);
  let entityBeingModified = $state<number | null>(null);

  // New condition form state
  let newConditionType = $state<string | undefined>(undefined);
  let newConditionSource = $state<string>("");
  let newConditionCause = $state<string>("");
  let newConditionSaveDC = $state<number | undefined>(undefined);
  let newConditionSaveAttribute = $state<string | undefined>(undefined);

  // Entity reminder management form state (turn-based only)
  let entityReminderName = $state<string | undefined>(undefined);
  let entityReminderDescription = $state<string | undefined>(undefined);
  let entityReminderTrigger = $state<string | undefined>(undefined);

  // Add entity form state
  let playerForm: PlayerEntity = $state(
    PlayerEntityActions.EmptyPlayerEntity(),
  );
  let monsterName: string | undefined = $state(undefined);
  let monsterId: string | undefined = $state(undefined);
  let repeatMonster: number | undefined = $state(undefined);
  let monsterRollStrategy: RollStrategy = $state(RollStrategy.Random);
  let reminderName: string | undefined = $state(undefined);
  let reminderType: "initiative" | "round" | undefined = $state(undefined);
  let reminderDescription: string | undefined = $state(undefined);
  let initiativeReminder: InitiativeReminder = $state(
    ReminderActions.EmptyInitiativeReminder(),
  );
  let roundReminder: RoundReminder = $state(
    ReminderActions.EmptyRoundReminder(),
  );

  // Monster list for selection
  let monsters: Monster[] = $state([]);
  let selectMonsters = $derived(
    monsters.map((monster) => ({
      label: monster.name!,
      value: monster.id!,
    })),
  );
  let selectedMonster = $derived(
    monsters.find((monster) => monster.id === monsterId),
  );

  // Damage type options for the select dropdown
  const damageTypeOptions = DamageTypes.map((type) => ({
    value: type,
    label: DisplayDamageType[type],
  }));

  // Condition options for the select dropdown
  const conditionOptions = Conditions.map((condition) => ({
    value: condition,
    label: DisplayCondition[condition],
  }));

  // Attribute options for the select dropdown
  const attributeOptions = Attributes.map((attribute) => ({
    value: attribute,
    label: DisplayAttribute[attribute],
  }));

  // Reminder type options
  const reminderTypeOptions = ToLabelValueWith(
    ReminderTypes,
    DisplayReminderType,
  );
  const roundTriggerOptions = ToLabelValueWith(RoundTriggers, DisplayTrigger);
  const turnTriggerOptions = ToLabelValueWith(TurnTriggers, DisplayTrigger);
  const rollStrategyOptions = ToLabelValueWith(
    RollStrategies,
    DisplayRollStrategy,
  );

  // Load monsters and encounter on mount
  onMount(async () => {
    console.log("EncounterView mounted");
    console.log("Current URL:", window.location.pathname);
    console.log("Extracted encounter ID:", encounterId);

    monsters = await MonsterService.GetAll();

    // Load encounter from backend if ID is provided
    if (encounterId) {
      console.log(`Fetching encounter from: /api/encounter/${encounterId}`);
      try {
        const response = await fetch(`/api/encounter/${encounterId}`);
        console.log("Fetch response status:", response.status);
        if (response.ok) {
          encounter = await response.json();
          console.log("Loaded encounter:", encounter);
          console.log("Monsters map from DB:", encounter.monsters);
          console.log(
            "Monster entities:",
            encounter.entities.filter((e) => e.type === "monster"),
          );
        } else {
          const errorText = await response.text();
          console.error(
            "Failed to fetch encounter:",
            response.status,
            errorText,
          );
          toast.error("Failed to load encounter");
        }
      } catch (error) {
        console.error("Failed to load encounter:", error);
        toast.error("Failed to load encounter");
      }
    } else {
      console.warn("No encounter ID found in URL!");
    }
    loading = false;
  });

  // Auto-save encounter state
  const { isSaving } = useAutoSave(() => encounter, "/api/encounter", 2000);

  // Get current active entity (for turn tracking)
  let activeEntity = $derived(sortedEntities[encounter.active] ?? null);

  // Get selected entity for viewing
  let selectedEntity = $derived(sortedEntities[selectedEntityIndex] ?? null);

  // Advance to next turn
  const nextTurn = () => {
    const wasAtEnd = encounter.active === sortedEntities.length - 1;
    encounter.active = (encounter.active + 1) % sortedEntities.length;
    selectedEntityIndex = encounter.active;

    // Increment round when we loop back to start
    if (wasAtEnd) {
      currentRound++;
    }
  };

  // Go to previous turn
  const previousTurn = () => {
    const wasAtStart = encounter.active === 0;
    encounter.active =
      encounter.active === 0 ? sortedEntities.length - 1 : encounter.active - 1;
    selectedEntityIndex = encounter.active;

    // Decrement round when we loop back from start
    if (wasAtStart && currentRound > 1) {
      currentRound--;
    }
  };

  // Add entity handlers
  const addPlayer = () => {
    if (!playerForm.name || playerForm.initiative === undefined) {
      toast.error("Please fill in name and initiative");
      return;
    }

    encounter.entities.push(playerForm);
    encounter.entities.sort(CompareEncounterEntities);
    playerForm = PlayerEntityActions.EmptyPlayerEntity();
    toast.success("Player added to encounter");
  };

  const addMonster = () => {
    if (!monsterName || !selectedMonster) {
      toast.error("Please fill in monster name and select a monster");
      return;
    }

    const addSingleMonster = (name: string) => {
      let monsterEntity = MonsterEntityActions.EmptyMonsterEntity();
      monsterEntity.name = name;
      monsterEntity.monster_id = selectedMonster.id;

      console.log(
        "Adding monster to encounter:",
        name,
        "with ID:",
        selectedMonster.id,
      );

      // Add monster to the encounter's monsters map if not already there
      if (selectedMonster.id && !encounter.monsters[selectedMonster.id]) {
        encounter.monsters[selectedMonster.id] = selectedMonster;
        console.log("Added monster to encounters map:", selectedMonster.name);
      } else {
        console.log("Monster already in map or no ID");
      }

      const hp = Roll(
        selectedMonster.rollable_hit_points!,
        monsterRollStrategy,
      );
      monsterEntity.max_hp = hp;
      monsterEntity.current_hp = hp;
      monsterEntity.initiative = new DiceRoll(
        `1d20+${GetModifier(selectedMonster.dexterity!)}`,
      ).total;
      encounter.entities.push(monsterEntity);
    };

    if (repeatMonster && repeatMonster > 1) {
      for (let i = 0; i < repeatMonster; i++) {
        addSingleMonster(`${monsterName} #${i + 1}`);
      }
    } else {
      addSingleMonster(monsterName);
    }

    encounter.entities.sort(CompareEncounterEntities);
    monsterName = undefined;
    monsterId = undefined;
    repeatMonster = undefined;
    monsterRollStrategy = RollStrategy.Random;
    toast.success("Monster(s) added to encounter");
  };

  const addReminder = () => {
    if (!reminderName || !reminderType || !reminderDescription) {
      toast.error("Please fill in name, type, and description");
      return;
    }

    let reminder: InitiativeReminder | RoundReminder;

    if (reminderType === "initiative") {
      if (initiativeReminder.initiative === undefined) {
        toast.error("Please set initiative for initiative reminder");
        return;
      }
      reminder = {
        ...initiativeReminder,
        name: reminderName,
        description: reminderDescription,
      };
    } else {
      if (!roundReminder.trigger) {
        toast.error("Please select trigger for round reminder");
        return;
      }
      reminder = {
        ...roundReminder,
        name: reminderName,
        description: reminderDescription,
      };
    }

    encounter.entities.push(reminder);
    encounter.entities.sort(CompareEncounterEntities);
    reminderName = undefined;
    reminderType = undefined;
    reminderDescription = undefined;
    initiativeReminder = ReminderActions.EmptyInitiativeReminder();
    roundReminder = ReminderActions.EmptyRoundReminder();
    toast.success("Reminder added to encounter");
  };

  // Entity action handlers
  const handleToggleConcentration = (index: number) => {
    const entity = sortedEntities[index];
    if (entity.type !== "reminder") {
      entity.concentration = !entity.concentration;
      toast.success(
        `${entity.name ?? "Entity"} ${entity.concentration ? "is now" : "is no longer"} concentrating`,
      );
    }
  };

  const handleManageConditions = (index: number) => {
    entityBeingModified = index;
    // Reset form
    newConditionType = undefined;
    newConditionSource = "";
    newConditionCause = "";
    newConditionSaveDC = undefined;
    newConditionSaveAttribute = undefined;
    conditionDialogOpen = true;
  };

  const addCondition = () => {
    if (entityBeingModified === null || !newConditionType) return;

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "reminder") return;

    const newCondition: CombatCondition = {
      condition: newConditionType as Condition,
      source: newConditionSource || undefined,
      cause: newConditionCause || undefined,
      saving_throw_dc: newConditionSaveDC,
      saving_throw_attribute: newConditionSaveAttribute as
        | Attribute
        | undefined,
    };

    entity.conditions.push(newCondition);

    toast.success(
      `Added ${DisplayCondition[newConditionType as Condition]} condition`,
    );

    // Reset form
    newConditionType = undefined;
    newConditionSource = "";
    newConditionCause = "";
    newConditionSaveDC = undefined;
    newConditionSaveAttribute = undefined;
  };

  const removeCondition = (conditionIndex: number) => {
    if (entityBeingModified === null) return;

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "reminder") return;

    const removed = entity.conditions[conditionIndex];
    entity.conditions.splice(conditionIndex, 1);

    toast.success(
      `Removed ${removed.condition ? DisplayCondition[removed.condition] : "condition"}`,
    );
  };

  const handleManageReminders = (index: number) => {
    entityBeingModified = index;
    // Reset form
    entityReminderName = undefined;
    entityReminderDescription = undefined;
    entityReminderTrigger = undefined;
    reminderManagementDialogOpen = true;
  };

  const addEntityReminder = () => {
    if (
      entityBeingModified === null ||
      !entityReminderName ||
      !entityReminderDescription ||
      !entityReminderTrigger
    ) {
      toast.error("Please fill in all required fields");
      return;
    }

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "reminder") return;

    const reminder: CreatureReminder = {
      id: uuid(),
      type: "reminder",
      name: entityReminderName,
      description: entityReminderDescription,
      trigger: entityReminderTrigger as Trigger.StartOfTurn | Trigger.EndOfTurn,
    };

    entity.reminders.push(reminder);
    toast.success(`Added reminder: ${entityReminderName}`);

    // Reset form
    entityReminderName = undefined;
    entityReminderDescription = undefined;
    entityReminderTrigger = undefined;
  };

  const removeEntityReminder = (reminderIndex: number) => {
    if (entityBeingModified === null) return;

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "reminder") return;

    const removed = entity.reminders[reminderIndex];
    entity.reminders.splice(reminderIndex, 1);

    toast.success(`Removed reminder: ${removed.name ?? "Reminder"}`);
  };

  const handleTakeDamage = (index: number) => {
    entityBeingModified = index;
    damageAmount = undefined;
    damageType = undefined;
    damageDialogOpen = true;
  };

  const handleHeal = (index: number) => {
    entityBeingModified = index;
    healAmount = undefined;
    healDialogOpen = true;
  };

  const applyDamage = () => {
    if (entityBeingModified === null || !damageAmount || !damageType) return;

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "monster") {
      const monsterEntity = entity as MonsterEntity;
      const monsterData = monsterEntity.monster_id
        ? encounter.monsters[monsterEntity.monster_id]
        : null;
      let actualDamage = damageAmount;
      let damageMessage = "";

      // Check for immunity (no damage)
      if (monsterData?.damage_immunities?.includes(damageType as DamageType)) {
        actualDamage = 0;
        damageMessage = `${monsterEntity.name ?? "Monster"} is immune to ${DisplayDamageType[damageType as DamageType]} damage!`;
      }
      // Check for resistance (half damage, rounded down)
      else if (
        monsterData?.damage_resistances?.includes(damageType as DamageType)
      ) {
        actualDamage = Math.floor(damageAmount / 2);
        damageMessage = `${monsterEntity.name ?? "Monster"} resisted ${DisplayDamageType[damageType as DamageType]} damage and took ${actualDamage} damage (${damageAmount} halved)`;
      }
      // Normal damage
      else {
        damageMessage = `${monsterEntity.name ?? "Monster"} took ${actualDamage} ${DisplayDamageType[damageType as DamageType]} damage`;
      }

      const newHp = Math.max(0, (monsterEntity.current_hp ?? 0) - actualDamage);
      monsterEntity.current_hp = newHp;

      if (actualDamage === 0) {
        toast.info(damageMessage);
      } else {
        toast.success(damageMessage);
      }

      damageDialogOpen = false;
    }
  };

  const applyHeal = () => {
    if (entityBeingModified === null || !healAmount) return;

    const entity = sortedEntities[entityBeingModified];
    if (entity.type === "monster") {
      const monster = entity as MonsterEntity;
      const newHp = Math.min(
        monster.max_hp ?? 0,
        (monster.current_hp ?? 0) + healAmount,
      );
      monster.current_hp = newHp;

      toast.success(`${monster.name ?? "Monster"} healed ${healAmount} HP`);
      healDialogOpen = false;
    }
  };
</script>

<Container class="mx-auto h-screen max-w-[1000px] py-4">
  {#if loading}
    <div class="flex h-full items-center justify-center">
      <p class="text-muted-foreground">Loading encounter...</p>
    </div>
  {:else if !encounterId}
    <div class="flex h-full items-center justify-center">
      <p class="text-muted-foreground">No encounter ID provided</p>
    </div>
  {:else}
    <div class="mb-4 flex items-center justify-between">
      <div>
        <Title variant="default">{encounter.name ?? "Unnamed Encounter"}</Title>
        <p class="text-muted-foreground ml-1 text-sm">Round {currentRound}</p>
      </div>
      <div class="flex items-center gap-3">
        {#if isSaving}
          <span class="text-primary text-sm font-medium">Saving...</span>
        {/if}
        <div class="flex gap-2">
          <Button
            variant="outline"
            onclick={() => (addEntityDialogOpen = true)}
          >
            <Plus class="mr-2 h-4 w-4" />
            Add Entity
          </Button>
          <Button
            variant="outline"
            onclick={() => (battleMapDialogOpen = true)}
          >
            <Map class="mr-2 h-4 w-4" />
            Battle Map
          </Button>
          <Button variant="outline" onclick={previousTurn}>Previous</Button>
          <Button onclick={nextTurn}>
            Next Turn
            <ChevronRight class="ml-2 h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>

    <div class="grid h-[calc(100vh-120px)] grid-cols-12 gap-4">
      <!-- Left Side: Initiative Order -->
      <div class="col-span-4">
        <InitiativeList
          entities={sortedEntities}
          activeIndex={encounter.active}
          selectedIndex={selectedEntityIndex}
          onSelectEntity={(index) => (selectedEntityIndex = index)}
          onToggleConcentration={handleToggleConcentration}
          onManageConditions={handleManageConditions}
          onManageReminders={handleManageReminders}
          onTakeDamage={handleTakeDamage}
          onHeal={handleHeal}
        />
      </div>

      <!-- Right Side: Entity Details -->
      <div class="col-span-8">
        {#if selectedEntity}
          <EntityDetailsPanel
            entity={selectedEntity}
            monsters={encounter.monsters}
          />
        {:else}
          <Container class="flex h-full items-center justify-center">
            <p class="text-muted-foreground">No entity selected</p>
          </Container>
        {/if}
      </div>
    </div>
  {/if}
</Container>

<!-- Damage Dialog -->
<Dialog.Root bind:open={damageDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Take Damage</Dialog.Title>
    </Dialog.Header>
    <div class="space-y-4">
      <div>
        <Label required>Damage Amount</Label>
        <Input
          type="number"
          bind:value={damageAmount}
          placeholder="Enter damage amount"
        />
      </div>
      <div>
        <Label required>Damage Type</Label>
        <Select
          bind:value={damageType}
          placeholder="Select damage type"
          items={damageTypeOptions}
        />
      </div>
      <div class="flex justify-end gap-2">
        <Button variant="outline" onclick={() => (damageDialogOpen = false)}>
          Cancel
        </Button>
        <Button
          onclick={applyDamage}
          disabled={!damageAmount || damageAmount <= 0 || !damageType}
        >
          Apply
        </Button>
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>

<!-- Heal Dialog -->
<Dialog.Root bind:open={healDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Heal</Dialog.Title>
    </Dialog.Header>
    <div class="space-y-4">
      <div>
        <Label required>Heal Amount</Label>
        <Input
          type="number"
          bind:value={healAmount}
          placeholder="Enter heal amount"
        />
      </div>
      <div class="flex justify-end gap-2">
        <Button variant="outline" onclick={() => (healDialogOpen = false)}>
          Cancel
        </Button>
        <Button onclick={applyHeal} disabled={!healAmount || healAmount <= 0}>
          Apply
        </Button>
      </div>
    </div>
  </Dialog.Content>
</Dialog.Root>

<!-- Condition Management Dialog -->
<Dialog.Root bind:open={conditionDialogOpen}>
  <Dialog.Content class="max-h-[90vh] max-w-2xl">
    <Dialog.Header>
      <Dialog.Title>Manage Conditions</Dialog.Title>
    </Dialog.Header>
    <ScrollArea class="max-h-[70vh]">
      <div class="space-y-6 pr-4">
        <!-- Add New Condition Form -->
        <div class="space-y-4">
          <h3 class="font-semibold">Add New Condition</h3>
          <div class="space-y-3">
            <div>
              <Label required>Condition</Label>
              <Select
                bind:value={newConditionType}
                placeholder="Select condition"
                items={conditionOptions}
              />
            </div>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <Label>Source</Label>
                <Input
                  type="text"
                  bind:value={newConditionSource}
                  placeholder="e.g., Gandalf, Dragon"
                />
              </div>
              <div>
                <Label>Cause</Label>
                <Input
                  type="text"
                  bind:value={newConditionCause}
                  placeholder="e.g., Fear spell, Poison"
                />
              </div>
            </div>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <Label>Save DC</Label>
                <Input
                  type="number"
                  bind:value={newConditionSaveDC}
                  placeholder="e.g., 15"
                />
              </div>
              <div>
                <Label>Save Attribute</Label>
                <Select
                  bind:value={newConditionSaveAttribute}
                  placeholder="Select attribute"
                  items={attributeOptions}
                />
              </div>
            </div>
            <Button
              onclick={addCondition}
              disabled={!newConditionType}
              class="w-full"
            >
              Add Condition
            </Button>
          </div>
        </div>

        <Separator />

        <!-- Current Conditions List -->
        <div class="space-y-4">
          <h3 class="font-semibold">Current Conditions</h3>
          {#if entityBeingModified !== null && sortedEntities[entityBeingModified].type !== "reminder"}
            {@const entity = sortedEntities[entityBeingModified]}
            {#if entity.type !== "reminder" && entity.conditions && entity.conditions.length > 0}
              <div class="space-y-2">
                {#each entity.conditions as condition, index}
                  <div class="bg-card rounded-lg border p-3">
                    <div class="flex items-start justify-between">
                      <div class="flex-1 space-y-1">
                        <div class="font-semibold">
                          {condition.condition
                            ? DisplayCondition[condition.condition]
                            : "Unknown"}
                        </div>
                        <div class="text-muted-foreground space-y-0.5 text-sm">
                          {#if condition.source}
                            <div>Source: {condition.source}</div>
                          {/if}
                          {#if condition.cause}
                            <div>Cause: {condition.cause}</div>
                          {/if}
                          {#if condition.saving_throw_dc}
                            <div>
                              Save DC: {condition.saving_throw_dc}
                              {#if condition.saving_throw_attribute}
                                ({DisplayAttribute[
                                  condition.saving_throw_attribute
                                ]})
                              {/if}
                            </div>
                          {/if}
                        </div>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        onclick={() => removeCondition(index)}
                      >
                        <X class="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <div
                class="text-muted-foreground rounded-lg border border-dashed p-4 text-center text-sm"
              >
                No conditions currently active
              </div>
            {/if}
          {/if}
        </div>
      </div>
    </ScrollArea>
    <div class="mt-4 flex justify-end">
      <Button variant="outline" onclick={() => (conditionDialogOpen = false)}>
        Close
      </Button>
    </div>
  </Dialog.Content>
</Dialog.Root>

<!-- Reminder Management Dialog -->
<Dialog.Root bind:open={reminderManagementDialogOpen}>
  <Dialog.Content class="max-h-[90vh] max-w-2xl">
    <Dialog.Header>
      <Dialog.Title>Manage Reminders</Dialog.Title>
    </Dialog.Header>
    <ScrollArea class="max-h-[70vh]">
      <div class="space-y-6 pr-4">
        <!-- Add New Reminder Form -->
        <div class="space-y-4">
          <h3 class="font-semibold">Add New Reminder</h3>
          <div class="space-y-3">
            <div>
              <Label required>Name</Label>
              <Input
                type="text"
                bind:value={entityReminderName}
                placeholder="Reminder name"
              />
            </div>
            <div>
              <Label required>Trigger</Label>
              <Select
                bind:value={entityReminderTrigger}
                placeholder="Select trigger"
                items={turnTriggerOptions}
              />
            </div>
            <div>
              <Label required>Description</Label>
              <TextArea
                bind:value={entityReminderDescription}
                placeholder="What should you be reminded of?"
              />
            </div>
            <Button onclick={addEntityReminder} class="w-full">
              Add Reminder
            </Button>
          </div>
        </div>

        <Separator />

        <!-- Current Reminders List -->
        <div class="space-y-4">
          <h3 class="font-semibold">Current Reminders</h3>
          {#if entityBeingModified !== null && sortedEntities[entityBeingModified].type !== "reminder"}
            {@const entity = sortedEntities[entityBeingModified]}
            {#if entity.type !== "reminder" && entity.reminders && entity.reminders.length > 0}
              <div class="space-y-2">
                {#each entity.reminders as reminder, index}
                  <div class="bg-card rounded-lg border p-3">
                    <div class="flex items-start justify-between">
                      <div class="flex-1 space-y-1">
                        <div class="font-semibold">
                          {reminder.name ?? "Unnamed Reminder"}
                        </div>
                        <div class="text-muted-foreground space-y-0.5 text-sm">
                          {#if reminder.trigger}
                            <div>
                              Trigger: {DisplayTrigger[reminder.trigger]}
                            </div>
                          {/if}
                          {#if reminder.description}
                            <div>{reminder.description}</div>
                          {/if}
                        </div>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        onclick={() => removeEntityReminder(index)}
                      >
                        <X class="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <div
                class="text-muted-foreground rounded-lg border border-dashed p-4 text-center text-sm"
              >
                No reminders currently active
              </div>
            {/if}
          {/if}
        </div>
      </div>
    </ScrollArea>
    <div class="mt-4 flex justify-end">
      <Button
        variant="outline"
        onclick={() => (reminderManagementDialogOpen = false)}
      >
        Close
      </Button>
    </div>
  </Dialog.Content>
</Dialog.Root>

<!-- Add Entity Dialog -->
<Dialog.Root bind:open={addEntityDialogOpen}>
  <Dialog.Content class="max-h-[90vh] max-w-3xl">
    <Dialog.Header>
      <Dialog.Title>Add Entity to Encounter</Dialog.Title>
    </Dialog.Header>
    <ScrollArea class="max-h-[75vh]">
      <Tabs.Root value="player" class="w-full">
        <Tabs.List class="grid w-full grid-cols-3">
          <Tabs.Trigger value="player">Player</Tabs.Trigger>
          <Tabs.Trigger value="monster">Monster</Tabs.Trigger>
          <Tabs.Trigger value="reminder">Reminder</Tabs.Trigger>
        </Tabs.List>

        <!-- Player Tab -->
        <Tabs.Content value="player" class="space-y-4 pr-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <Label required>Name</Label>
              <Input
                type="text"
                bind:value={playerForm.name}
                placeholder="Player name"
              />
            </div>
            <div>
              <Label required>Initiative</Label>
              <div class="flex gap-2">
                <Input
                  type="number"
                  bind:value={playerForm.initiative}
                  placeholder="15"
                  class="text-center"
                />
                <Button
                  variant="outline"
                  onclick={() =>
                    (playerForm.initiative = new DiceRoll("1d20").total)}
                >
                  <Dices class="h-4 w-4" />
                </Button>
              </div>
            </div>
          </div>
          <Button onclick={addPlayer} class="w-full">Add Player</Button>
        </Tabs.Content>

        <!-- Monster Tab -->
        <Tabs.Content value="monster" class="space-y-4 pr-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <Label required>Name</Label>
              <Input
                type="text"
                bind:value={monsterName}
                placeholder="Monster name"
              />
            </div>
            <div>
              <Label required>Monster</Label>
              <Combobox
                placeholder="Select a monster"
                items={selectMonsters}
                bind:value={monsterId}
              />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <Label>Number to Add</Label>
              <Input
                type="number"
                bind:value={repeatMonster}
                placeholder="1"
                class="text-center"
              />
            </div>
            <div>
              <Label>HP Roll Strategy</Label>
              <Select
                bind:value={monsterRollStrategy}
                placeholder="Select strategy"
                items={rollStrategyOptions}
              />
            </div>
          </div>
          {#if selectedMonster}
            <div class="bg-muted rounded-lg border p-3">
              <div class="text-sm">
                <div class="font-semibold">{selectedMonster.name}</div>
                <div class="text-muted-foreground">
                  CR {selectedMonster.challenge_rating} • AC {selectedMonster.armor_class}
                  • HP {selectedMonster.hit_points}
                </div>
              </div>
            </div>
          {/if}
          <Button onclick={addMonster} class="w-full">Add Monster</Button>
        </Tabs.Content>

        <!-- Reminder Tab -->
        <Tabs.Content value="reminder" class="space-y-4 pr-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <Label required>Name</Label>
              <Input
                type="text"
                bind:value={reminderName}
                placeholder="Reminder name"
              />
            </div>
            <div>
              <Label required>Type</Label>
              <Select
                bind:value={reminderType}
                placeholder="Select type"
                items={reminderTypeOptions}
              />
            </div>
          </div>
          <div>
            <Label required>Description</Label>
            <TextArea
              bind:value={reminderDescription}
              placeholder="What should you be reminded of?"
            />
          </div>
          {#if reminderType === "initiative"}
            <div>
              <Label required>Initiative</Label>
              <div class="flex gap-2">
                <Input
                  type="number"
                  bind:value={initiativeReminder.initiative}
                  placeholder="15"
                  class="text-center"
                />
                <Button
                  variant="outline"
                  onclick={() =>
                    (initiativeReminder.initiative = new DiceRoll(
                      "1d20",
                    ).total)}
                >
                  <Dices class="h-4 w-4" />
                </Button>
              </div>
            </div>
          {:else if reminderType === "round"}
            <div>
              <Label required>Trigger</Label>
              <Select
                bind:value={roundReminder.trigger}
                placeholder="Select trigger"
                items={roundTriggerOptions}
              />
            </div>
          {/if}
          <Button onclick={addReminder} class="w-full" disabled={!reminderType}>
            Add Reminder
          </Button>
        </Tabs.Content>
      </Tabs.Root>
    </ScrollArea>
    <div class="mt-4 flex justify-end">
      <Button variant="outline" onclick={() => (addEntityDialogOpen = false)}>
        Close
      </Button>
    </div>
  </Dialog.Content>
</Dialog.Root>

<!-- Battle Map Dialog -->
<Dialog.Root bind:open={battleMapDialogOpen}>
  <Dialog.Content class="sm:max-w-none w-fit max-w-[96vw]">
    <Dialog.Header>
      <Dialog.Title>Battle Map</Dialog.Title>
      <Dialog.Description>
        Manage entity positions and terrain on the battle map
      </Dialog.Description>
    </Dialog.Header>
    <BattleMap {encounter} />
    <div class="mt-4 flex justify-end">
      <Button variant="outline" onclick={() => (battleMapDialogOpen = false)}>
        Close
      </Button>
    </div>
  </Dialog.Content>
</Dialog.Root>
