<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "@mateothegreat/svelte5-router";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import Plus from "@lucide/svelte/icons/plus";
  import Swords from "@lucide/svelte/icons/swords";
  import BookOpen from "@lucide/svelte/icons/book-open";
  import Sparkles from "@lucide/svelte/icons/sparkles";
  import List from "@lucide/svelte/icons/list";
  import type { Encounter, Monster, Spell } from "$types";

  let props = $props();

  let encounters = $state<Encounter[]>([]);
  let monsters = $state<Monster[]>([]);
  let spells = $state<Spell[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const [encountersRes, monstersRes, spellsRes] = await Promise.all([
        fetch("/api/encounters"),
        fetch("/api/monsters"),
        fetch("/api/spells"),
      ]);

      if (encountersRes.ok) encounters = await encountersRes.json();
      if (monstersRes.ok) monsters = await monstersRes.json();
      if (spellsRes.ok) spells = await spellsRes.json();
    } catch (error) {
      console.error("Failed to load data:", error);
    } finally {
      loading = false;
    }
  });

  const recentEncounters = $derived(
    encounters.slice().sort((a, b) => {
      // Sort by name if no other criteria available
      return (a.name ?? "").localeCompare(b.name ?? "");
    }).slice(0, 3)
  );
</script>

<Container class="mx-auto max-w-[1000px] py-8">
  <div class="space-y-8">
    <!-- Welcome Header -->
    <div class="text-center">
      <Title variant="default" class="mb-2">Initiative Tracker</Title>
      <p class="text-muted-foreground">
        Manage your D&D 5e encounters, monsters, and spells
      </p>
    </div>

    <!-- Overview -->
    {#if !loading}
      <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
        <!-- Encounters Card -->
        <div class="group rounded-lg border bg-card p-6 transition-colors">
          <div class="space-y-4">
            <div class="flex items-center gap-4">
              <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10 group-hover:bg-primary/20 transition-colors">
                <Swords class="h-6 w-6 text-primary" />
              </div>
              <div>
                <p class="text-2xl font-bold">{encounters.length}</p>
                <p class="text-sm text-muted-foreground">Encounters</p>
              </div>
            </div>
            <div class="flex gap-2">
              <Button size="sm" onclick={() => goto("/encounters/create")}>
                <Plus class="mr-2 h-4 w-4" />
                New
              </Button>
              <Button size="sm" variant="outline" onclick={() => goto("/encounters")}>
                <List class="mr-2 h-4 w-4" />
                View All
              </Button>
            </div>
          </div>
        </div>

        <!-- Monsters Card -->
        <div class="group rounded-lg border bg-card p-6 transition-colors">
          <div class="space-y-4">
            <div class="flex items-center gap-4">
              <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-destructive/10 group-hover:bg-destructive/20 transition-colors">
                <BookOpen class="h-6 w-6 text-destructive" />
              </div>
              <div>
                <p class="text-2xl font-bold">{monsters.length}</p>
                <p class="text-sm text-muted-foreground">Monsters</p>
              </div>
            </div>
            <div class="flex gap-2">
              <Button size="sm" onclick={() => goto("/monsters/create")}>
                <Plus class="mr-2 h-4 w-4" />
                New
              </Button>
              <Button size="sm" variant="outline" onclick={() => goto("/monsters")}>
                <List class="mr-2 h-4 w-4" />
                View All
              </Button>
            </div>
          </div>
        </div>

        <!-- Spells Card -->
        <div class="group rounded-lg border bg-card p-6 transition-colors">
          <div class="space-y-4">
            <div class="flex items-center gap-4">
              <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-500/10 group-hover:bg-blue-500/20 transition-colors">
                <Sparkles class="h-6 w-6 text-blue-500" />
              </div>
              <div>
                <p class="text-2xl font-bold">{spells.length}</p>
                <p class="text-sm text-muted-foreground">Spells</p>
              </div>
            </div>
            <div class="flex gap-2">
              <Button size="sm" onclick={() => goto("/spells/create")}>
                <Plus class="mr-2 h-4 w-4" />
                New
              </Button>
              <Button size="sm" variant="outline" onclick={() => goto("/spells")}>
                <List class="mr-2 h-4 w-4" />
                View All
              </Button>
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Recent Encounters -->
    {#if !loading && recentEncounters.length > 0}
      <div>
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-xl font-semibold">Recent Encounters</h2>
          <Button variant="ghost" size="sm" onclick={() => goto("/encounters")}>
            View All
          </Button>
        </div>
        <div class="space-y-2">
          {#each recentEncounters as encounter}
            <div
              class="rounded-lg border bg-card p-4 hover:bg-accent transition-colors cursor-pointer"
              onclick={() => goto(`/encounters/view/${encounter.id}`)}
            >
              <div class="flex items-center justify-between">
                <div>
                  <h3 class="font-semibold">{encounter.name ?? "Unnamed Encounter"}</h3>
                  <p class="text-sm text-muted-foreground">
                    {encounter.entities?.length ?? 0} entities
                  </p>
                </div>
                <Button variant="outline" size="sm">
                  View
                </Button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Container>
