<script lang="ts">
  import { onMount } from "svelte";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { goto } from "@mateothegreat/svelte5-router";
  import Eye from "@lucide/svelte/icons/eye";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import type { Spell } from "$types";
  import {
    Class,
    DisplayClass,
    DisplayMagicSchool,
    DisplaySpellLevel,
    MagicSchool,
    MagicSchools,
    SpellLevel,
    SpellcastingClasses,
  } from "$types";
  import { toast } from "svelte-sonner";

  let props = $props();

  let spells = $state<Spell[]>([]);
  let loading = $state(true);
  let sortBy = $state<"name" | "level" | "school">("name");
  let sortDirection = $state<"asc" | "desc">("asc");
  let filterSchool = $state<string>("all");
  let filterLevel = $state<string>("all");
  let filterClass = $state<string>("all");

  onMount(async () => {
    try {
      const response = await fetch("/api/spells");
      if (response.ok) {
        spells = await response.json();
      }
    } catch (error) {
      console.error("Failed to load spells:", error);
      toast.error("Failed to load spells");
    } finally {
      loading = false;
    }
  });

  const deleteSpell = async (spellId: string, spellName: string) => {
    if (!confirm(`Are you sure you want to delete "${spellName}"?`)) {
      return;
    }

    try {
      const response = await fetch(`/api/spell/${spellId}`, {
        method: "DELETE",
      });

      if (response.ok || response.status === 204) {
        spells = spells.filter((s) => s.id !== spellId);
        toast.success("Spell deleted successfully");
      } else {
        toast.error("Failed to delete spell");
      }
    } catch (error) {
      console.error("Failed to delete spell:", error);
      toast.error("Failed to delete spell");
    }
  };

  const sortedAndFilteredSpells = $derived(() => {
    let filtered = spells;

    // Filter by school
    if (filterSchool !== "all") {
      filtered = filtered.filter((s) => s.school === filterSchool);
    }

    // Filter by level
    if (filterLevel !== "all") {
      filtered = filtered.filter((s) => s.level === filterLevel);
    }

    // Filter by class
    if (filterClass !== "all") {
      filtered = filtered.filter((s) => s.classes.includes(filterClass as Class));
    }

    // Sort
    const sorted = [...filtered].sort((a, b) => {
      let compareValue = 0;

      switch (sortBy) {
        case "name":
          compareValue = (a.name ?? "").localeCompare(b.name ?? "");
          break;
        case "level":
          const aLevel = Object.values(SpellLevel).indexOf(a.level ?? SpellLevel.Cantrip);
          const bLevel = Object.values(SpellLevel).indexOf(b.level ?? SpellLevel.Cantrip);
          compareValue = aLevel - bLevel;
          break;
        case "school":
          compareValue = (a.school ?? "").localeCompare(b.school ?? "");
          break;
      }

      return sortDirection === "asc" ? compareValue : -compareValue;
    });

    return sorted;
  });

  const toggleSort = (field: "name" | "level" | "school") => {
    if (sortBy === field) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      sortBy = field;
      sortDirection = "asc";
    }
  };

  const getSpellComponents = (spell: Spell): string => {
    const components: string[] = [];
    if (spell.verbal) components.push("V");
    if (spell.somatic) components.push("S");
    if (spell.material) components.push("M");
    return components.join(", ");
  };
</script>

<Container class="mx-auto max-w-[1000px] py-4">
  <div class="mb-4 flex items-center justify-between">
    <Title variant="default">Spells</Title>
    <Button onclick={() => goto("/spells/create")}>
      <Plus class="mr-2 h-4 w-4" />
      Create Spell
    </Button>
  </div>

  <!-- Filters and Sorting -->
  <div class="mb-4 space-y-2">
    <!-- Filters Row -->
    <div class="flex flex-wrap gap-2">
      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">School:</span>
        <select
          bind:value={filterSchool}
          class="rounded-md border bg-background px-3 py-1 text-sm"
        >
          <option value="all">All Schools</option>
          {#each MagicSchools as school}
            <option value={school}>{DisplayMagicSchool[school]}</option>
          {/each}
        </select>
      </div>

      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">Level:</span>
        <select
          bind:value={filterLevel}
          class="rounded-md border bg-background px-3 py-1 text-sm"
        >
          <option value="all">All Levels</option>
          {#each Object.values(SpellLevel) as level}
            <option value={level}>{DisplaySpellLevel[level]}</option>
          {/each}
        </select>
      </div>

      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">Class:</span>
        <select
          bind:value={filterClass}
          class="rounded-md border bg-background px-3 py-1 text-sm"
        >
          <option value="all">All Classes</option>
          {#each SpellcastingClasses as classType}
            <option value={classType}>{DisplayClass[classType]}</option>
          {/each}
        </select>
      </div>
    </div>

    <!-- Sorting Row -->
    <div class="flex items-center gap-2">
      <span class="text-sm text-muted-foreground">Sort by:</span>
      <Button
        variant={sortBy === "name" ? "default" : "outline"}
        size="sm"
        onclick={() => toggleSort("name")}
      >
        Name {sortBy === "name" ? (sortDirection === "asc" ? "↑" : "↓") : ""}
      </Button>
      <Button
        variant={sortBy === "level" ? "default" : "outline"}
        size="sm"
        onclick={() => toggleSort("level")}
      >
        Level {sortBy === "level" ? (sortDirection === "asc" ? "↑" : "↓") : ""}
      </Button>
      <Button
        variant={sortBy === "school" ? "default" : "outline"}
        size="sm"
        onclick={() => toggleSort("school")}
      >
        School {sortBy === "school" ? (sortDirection === "asc" ? "↑" : "↓") : ""}
      </Button>
    </div>
  </div>

  <div class="space-y-2">
    {#if loading}
      <div class="text-center text-muted-foreground">Loading spells...</div>
    {:else if sortedAndFilteredSpells().length === 0}
      <div class="mt-8 rounded-lg border border-dashed p-8 text-center text-muted-foreground">
        {#if filterSchool !== "all" || filterLevel !== "all" || filterClass !== "all"}
          <p>No spells found with the selected filters.</p>
        {:else}
          <p>No spells yet. Create your first spell to get started!</p>
        {/if}
      </div>
    {:else}
      {#each sortedAndFilteredSpells() as spell}
        <div
          class="flex items-center justify-between rounded-lg border bg-card p-4 transition-colors hover:bg-accent"
        >
          <div class="flex-1">
            <div class="flex items-center gap-2">
              <h3 class="font-semibold">{spell.name ?? "Unnamed Spell"}</h3>
              {#if spell.concentration}
                <span class="text-xs text-blue-500">(C)</span>
              {/if}
              {#if spell.ritual}
                <span class="text-xs text-purple-500">(R)</span>
              {/if}
            </div>
            <div class="flex flex-wrap gap-4 text-sm text-muted-foreground">
              <span>{spell.level ? DisplaySpellLevel[spell.level] : "—"}</span>
              <span>{spell.school ? DisplayMagicSchool[spell.school] : "—"}</span>
              <span>Components: {getSpellComponents(spell)}</span>
              <span>Range: {spell.range ?? "—"}</span>
              <span>Duration: {spell.duration ?? "—"}</span>
            </div>
            <div class="mt-1 flex flex-wrap gap-1">
              {#each spell.classes as classType}
                <span class="rounded-full bg-primary/20 px-2 py-0.5 text-xs font-medium text-primary">
                  {DisplayClass[classType]}
                </span>
              {/each}
            </div>
          </div>
          <div class="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onclick={() => goto(`/spells/view/${spell.id}`)}
            >
              <Eye class="mr-2 h-4 w-4" />
              View
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onclick={(e) => {
                e.stopPropagation();
                deleteSpell(spell.id!, spell.name ?? "Unnamed Spell");
              }}
            >
              <Trash class="h-4 w-4" />
            </Button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</Container>
