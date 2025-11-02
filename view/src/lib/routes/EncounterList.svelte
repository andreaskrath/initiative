<script lang="ts">
  import { onMount } from "svelte";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { goto } from "@mateothegreat/svelte5-router";
  import Eye from "@lucide/svelte/icons/eye";
  import Plus from "@lucide/svelte/icons/plus";
  import Trash from "@lucide/svelte/icons/trash-2";
  import type { Encounter } from "$types";
  import { toast } from "svelte-sonner";

  let props = $props();

  let encounters = $state<Encounter[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const response = await fetch("/api/encounters");
      if (response.ok) {
        encounters = await response.json();
        console.log("Loaded encounters:", encounters);
        encounters.forEach((enc, idx) => {
          console.log(`Encounter ${idx} id:`, enc.id);
        });
      }
    } catch (error) {
      console.error("Failed to load encounters:", error);
    } finally {
      loading = false;
    }
  });

  const deleteEncounter = async (encounterId: string, encounterName: string) => {
    if (!confirm(`Are you sure you want to delete "${encounterName}"?`)) {
      return;
    }

    try {
      const response = await fetch(`/api/encounter/${encounterId}`, {
        method: "DELETE",
      });

      if (response.ok || response.status === 204) {
        encounters = encounters.filter(e => e.id !== encounterId);
        toast.success("Encounter deleted successfully");
      } else {
        toast.error("Failed to delete encounter");
      }
    } catch (error) {
      console.error("Failed to delete encounter:", error);
      toast.error("Failed to delete encounter");
    }
  };
</script>

<Container class="mx-auto max-w-[1000px] py-4">
  <div class="mb-4 flex items-center justify-between">
    <Title variant="default">Encounters</Title>
    <Button onclick={() => goto("/encounters/create")}>
      <Plus class="mr-2 h-4 w-4" />
      Create Encounter
    </Button>
  </div>

  <div class="space-y-2">
    {#if loading}
      <div class="text-center text-muted-foreground">Loading encounters...</div>
    {:else if encounters.length === 0}
      <div class="mt-8 rounded-lg border border-dashed p-8 text-center text-muted-foreground">
        <p>No encounters yet. Create your first encounter to get started!</p>
      </div>
    {:else}
      {#each encounters as encounter}
        <div
          class="flex items-center justify-between rounded-lg border bg-card p-4 transition-colors hover:bg-accent"
        >
          <div>
            <h3 class="font-semibold">{encounter.name ?? "Unnamed Encounter"}</h3>
            <p class="text-sm text-muted-foreground">
              {encounter.entities?.length ?? 0} entities
            </p>
          </div>
          <div class="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onclick={() => goto(`/encounters/view/${encounter.id}`)}
            >
              <Eye class="mr-2 h-4 w-4" />
              View
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onclick={(e) => {
                e.stopPropagation();
                deleteEncounter(encounter.id!, encounter.name ?? "Unnamed Encounter");
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
