<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "@mateothegreat/svelte5-router";
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { ScrollArea } from "$components/ui/scroll-area/index";
  import { Separator } from "$components/ui/separator/index";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import Trash from "@lucide/svelte/icons/trash-2";
  import type { Spell } from "$types";
  import { DisplayClass, DisplayMagicSchool, DisplaySpellLevel } from "$types";
  import { toast } from "svelte-sonner";

  let props = $props();

  let spell = $state<Spell | null>(null);
  let loading = $state(true);

  const spellId = $derived(() => {
    const path = window.location.pathname;
    const match = path.match(/\/spells\/view\/([^/]+)/);
    return match ? match[1] : null;
  });

  onMount(async () => {
    const id = spellId();
    if (!id) {
      toast.error("Invalid spell ID");
      goto("/spells");
      return;
    }

    try {
      const response = await fetch(`/api/spell/${id}`);
      if (response.ok) {
        spell = await response.json();
      } else {
        toast.error("Spell not found");
        goto("/spells");
      }
    } catch (error) {
      console.error("Failed to load spell:", error);
      toast.error("Failed to load spell");
      goto("/spells");
    } finally {
      loading = false;
    }
  });

  const deleteSpell = async () => {
    if (!spell?.id) return;

    if (!confirm(`Are you sure you want to delete "${spell.name}"?`)) {
      return;
    }

    try {
      const response = await fetch(`/api/spell/${spell.id}`, {
        method: "DELETE",
      });

      if (response.ok || response.status === 204) {
        toast.success("Spell deleted successfully");
        goto("/spells");
      } else {
        toast.error("Failed to delete spell");
      }
    } catch (error) {
      console.error("Failed to delete spell:", error);
      toast.error("Failed to delete spell");
    }
  };
</script>

<Container class="mx-auto max-w-[900px] py-4">
  <ScrollArea class="h-[calc(100vh-120px)]">
    <div class="space-y-4 pr-4">
      {#if loading}
        <div class="text-center text-muted-foreground">Loading spell...</div>
      {:else if spell}
        <!-- Header with Actions -->
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-4">
            <Button variant="ghost" size="icon" onclick={() => goto("/spells")}>
              <ArrowLeft class="h-4 w-4" />
            </Button>
            <div>
              <div class="flex items-center gap-2">
                <Title variant="default">{spell.name ?? "Unnamed Spell"}</Title>
                {#if spell.concentration}
                  <span class="text-sm text-blue-500">(Concentration)</span>
                {/if}
                {#if spell.ritual}
                  <span class="text-sm text-purple-500">(Ritual)</span>
                {/if}
              </div>
              <p class="text-sm text-muted-foreground">
                {spell.level ? DisplaySpellLevel[spell.level] : "Unknown Level"}
                {spell.school ? ` • ${DisplayMagicSchool[spell.school]}` : ""}
              </p>
            </div>
          </div>
          <div class="flex gap-2">
            <Button variant="outline" onclick={deleteSpell}>
              <Trash class="mr-2 h-4 w-4" />
              Delete
            </Button>
          </div>
        </div>

        <Separator />

        <!-- Basic Info -->
        <div class="grid grid-cols-1 gap-4 text-sm md:grid-cols-2">
          <div>
            <h3 class="mb-1 font-semibold">Casting Time</h3>
            <p class="text-muted-foreground">
              {spell.casting_time ?? "—"}
              {#if spell.ritual}
                <span class="ml-1 text-xs text-purple-500">(or 10 minutes as ritual)</span>
              {/if}
            </p>
          </div>
          <div>
            <h3 class="mb-1 font-semibold">Range</h3>
            <p class="text-muted-foreground">{spell.range ?? "—"}</p>
          </div>
          <div>
            <h3 class="mb-1 font-semibold">Duration</h3>
            <p class="text-muted-foreground">
              {#if spell.concentration}
                <span class="text-blue-500">Concentration, </span>
              {/if}
              {spell.duration ?? "—"}
            </p>
          </div>
          {#if spell.area || spell.shape}
            <div>
              <h3 class="mb-1 font-semibold">Area</h3>
              <p class="text-muted-foreground">
                {spell.shape ? `${spell.shape}` : ""}
                {spell.area ? ` (${spell.area})` : ""}
              </p>
            </div>
          {/if}
        </div>

        <Separator />

        <!-- Components -->
        {#if spell.verbal || spell.somatic || spell.material}
          <div>
            <h3 class="mb-2 font-semibold">Components</h3>
            <div class="space-y-1 text-sm text-muted-foreground">
              {#if spell.verbal}
                <div>• Verbal (V)</div>
              {/if}
              {#if spell.somatic}
                <div>• Somatic (S)</div>
              {/if}
              {#if spell.material}
                <div>
                  • Material (M): {spell.material}
                  {#if spell.material_consumed}
                    <span class="ml-1 text-xs font-semibold text-destructive">(consumed)</span>
                  {/if}
                </div>
              {/if}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Classes -->
        {#if spell.classes && spell.classes.length > 0}
          <div>
            <h3 class="mb-2 font-semibold">Available to Classes</h3>
            <div class="flex flex-wrap gap-2">
              {#each spell.classes as classType}
                <span class="rounded-full bg-primary/20 px-3 py-1 text-sm font-medium text-primary">
                  {DisplayClass[classType]}
                </span>
              {/each}
            </div>
          </div>
          <Separator />
        {/if}

        <!-- Description -->
        {#if spell.description}
          <div>
            <h3 class="mb-2 font-semibold">Description</h3>
            <p class="whitespace-pre-wrap text-sm leading-relaxed text-muted-foreground">
              {spell.description}
            </p>
          </div>
          <Separator />
        {/if}

        <!-- At Higher Levels -->
        {#if spell.at_higher_levels}
          <div>
            <h3 class="mb-2 font-semibold">At Higher Levels</h3>
            <p class="whitespace-pre-wrap text-sm leading-relaxed text-muted-foreground">
              {spell.at_higher_levels}
            </p>
          </div>
        {/if}
      {/if}
    </div>
  </ScrollArea>
</Container>
