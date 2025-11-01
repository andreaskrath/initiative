<script lang="ts">
  import Container from "$components/Container.svelte";
  import Title from "$components/Title.svelte";
  import { Button } from "$components/ui/button/index";
  import { goto } from "@mateothegreat/svelte5-router";
  import Eye from "@lucide/svelte/icons/eye";
  import Plus from "@lucide/svelte/icons/plus";

  let props = $props();

  // Dummy encounter for testing - in real app this would come from API
  const dummyEncounter = {
    id: "test-encounter",
    name: "Goblin Ambush",
    description: "A test encounter with goblins",
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
    <!-- Dummy encounter card -->
    <div
      class="flex items-center justify-between rounded-lg border bg-card p-4 transition-colors hover:bg-accent"
    >
      <div>
        <h3 class="font-semibold">{dummyEncounter.name}</h3>
        <p class="text-sm text-muted-foreground">{dummyEncounter.description}</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" size="sm" onclick={() => goto("/encounters/view")}>
          <Eye class="mr-2 h-4 w-4" />
          View
        </Button>
      </div>
    </div>

    <!-- Empty state message -->
    <div class="mt-8 text-center text-muted-foreground">
      <p>This is a demo encounter. In a real app, this would load from the API.</p>
    </div>
  </div>
</Container>
