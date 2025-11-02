<script lang="ts">
  import { onMount } from "svelte";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { goto } from "@mateothegreat/svelte5-router";
  import Eye from "@lucide/svelte/icons/eye";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import type { Monster } from "$types";
  import { toast } from "svelte-sonner";

  let props = $props();

  let monsters = $state<Monster[]>([]);
  let loading = $state(true);
  let sortBy = $state<"name" | "cr" | "type">("name");
  let sortDirection = $state<"asc" | "desc">("asc");
  let filterType = $state<string>("all");

  onMount(async () => {
    try {
      const response = await fetch("/api/monsters");
      if (response.ok) {
        monsters = await response.json();
      }
    } catch (error) {
      console.error("Failed to load monsters:", error);
      toast.error("Failed to load monsters");
    } finally {
      loading = false;
    }
  });

  const deleteMonster = async (monsterId: string, monsterName: string) => {
    if (!confirm(`Are you sure you want to delete "${monsterName}"?`)) {
      return;
    }

    try {
      const response = await fetch(`/api/monster/${monsterId}`, {
        method: "DELETE",
      });

      if (response.ok || response.status === 204) {
        monsters = monsters.filter((m) => m.id !== monsterId);
        toast.success("Monster deleted successfully");
      } else {
        toast.error("Failed to delete monster");
      }
    } catch (error) {
      console.error("Failed to delete monster:", error);
      toast.error("Failed to delete monster");
    }
  };

  const sortedAndFilteredMonsters = $derived(() => {
    let filtered = monsters;

    // Filter by type
    if (filterType !== "all") {
      filtered = filtered.filter((m) => m.monster_type === filterType);
    }

    // Sort
    const sorted = [...filtered].sort((a, b) => {
      let compareValue = 0;

      switch (sortBy) {
        case "name":
          compareValue = (a.name ?? "").localeCompare(b.name ?? "");
          break;
        case "cr":
          compareValue = (a.challenge_rating ?? 0) - (b.challenge_rating ?? 0);
          break;
        case "type":
          compareValue = (a.monster_type ?? "").localeCompare(b.monster_type ?? "");
          break;
      }

      return sortDirection === "asc" ? compareValue : -compareValue;
    });

    return sorted;
  });

  const uniqueTypes = $derived(() => {
    const types = new Set(monsters.map((m) => m.monster_type).filter((t) => t));
    return Array.from(types).sort();
  });

  const toggleSort = (field: "name" | "cr" | "type") => {
    if (sortBy === field) {
      sortDirection = sortDirection === "asc" ? "desc" : "asc";
    } else {
      sortBy = field;
      sortDirection = "asc";
    }
  };
</script>

<Container class="mx-auto max-w-[1000px] py-4">
  <div class="mb-4 flex items-center justify-between">
    <Title variant="default">Monsters</Title>
    <Button onclick={() => goto("/monsters/create")}>
      <Plus class="mr-2 h-4 w-4" />
      Create Monster
    </Button>
  </div>

  <!-- Filters and Sorting -->
  <div class="mb-4 flex flex-wrap gap-2">
    <div class="flex items-center gap-2">
      <span class="text-sm text-muted-foreground">Filter:</span>
      <select
        bind:value={filterType}
        class="rounded-md border bg-background px-3 py-1 text-sm"
      >
        <option value="all">All Types</option>
        {#each uniqueTypes() as type}
          <option value={type}>{type}</option>
        {/each}
      </select>
    </div>

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
        variant={sortBy === "cr" ? "default" : "outline"}
        size="sm"
        onclick={() => toggleSort("cr")}
      >
        CR {sortBy === "cr" ? (sortDirection === "asc" ? "↑" : "↓") : ""}
      </Button>
      <Button
        variant={sortBy === "type" ? "default" : "outline"}
        size="sm"
        onclick={() => toggleSort("type")}
      >
        Type {sortBy === "type" ? (sortDirection === "asc" ? "↑" : "↓") : ""}
      </Button>
    </div>
  </div>

  <div class="space-y-2">
    {#if loading}
      <div class="text-center text-muted-foreground">Loading monsters...</div>
    {:else if sortedAndFilteredMonsters().length === 0}
      <div class="mt-8 rounded-lg border border-dashed p-8 text-center text-muted-foreground">
        {#if filterType !== "all"}
          <p>No monsters found with the selected filter.</p>
        {:else}
          <p>No monsters yet. Create your first monster to get started!</p>
        {/if}
      </div>
    {:else}
      {#each sortedAndFilteredMonsters() as monster}
        <div
          class="flex items-center justify-between rounded-lg border bg-card p-4 transition-colors hover:bg-accent"
        >
          <div class="flex-1">
            <h3 class="font-semibold">{monster.name ?? "Unnamed Monster"}</h3>
            <div class="flex gap-4 text-sm text-muted-foreground">
              <span>CR {monster.challenge_rating ?? "—"}</span>
              <span>{monster.monster_type ?? "Unknown Type"}</span>
              <span>{monster.size ?? "Unknown Size"}</span>
              <span>HP {monster.hit_points ?? "—"}</span>
              <span>AC {monster.armor_class ?? "—"}</span>
            </div>
          </div>
          <div class="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onclick={() => goto(`/monsters/view/${monster.id}`)}
            >
              <Eye class="mr-2 h-4 w-4" />
              View
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onclick={(e) => {
                e.stopPropagation();
                deleteMonster(monster.id!, monster.name ?? "Unnamed Monster");
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
