<script lang="ts">
  import "./app.css";
  import Monsters from "$lib/monsters/Monsters.svelte";
  import Encounters from "$lib/encounters/Encounters.svelte";
  import NotFound from "$lib/NotFound.svelte";
  import Settings from "$lib/settings/Settings.svelte";
  // import Spells from "$lib/spells/Spells.svelte";
  import Navbar from "$components/Navbar.svelte";
  import type { Component } from "svelte";

  import ModeToggle from "$components/ModeToggle.svelte";

  import { ModeWatcher } from "mode-watcher";

  const pages: Component[] = [Monsters, Encounters, Settings];
  let currentPage: number = $state(0);
  const PageComponent = $derived(pages[currentPage] || NotFound);
</script>

<Navbar
  bind:currentItem={currentPage}
  items={["Monsters", "Encounters", "Spells", "Settings"]}
/>
<main>
  <ModeWatcher />
  <ModeToggle />
  <div class="mx-auto w-[1200px] py-2">
    <PageComponent />
  </div>
</main>
