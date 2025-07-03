<script lang="ts">
  import "./app.css";
  import Monsters from "$lib/monsters/Monsters.svelte";
  import Encounters from "$lib/encounters/Encounters.svelte";
  import NotFound from "$lib/NotFound.svelte";
  import Settings from "$lib/settings/Settings.svelte";
  import Spells from "$lib/spells/Spells.svelte";
  import Navbar from "$components/Navbar.svelte";
  import type { Component } from "svelte";

  const pages: Component[] = [Monsters, Encounters, Spells, Settings];
  let currentPage: number = $state(0);
  const PageComponent = $derived(pages[currentPage] || NotFound);
</script>

<Navbar
  bind:currentItem={currentPage}
  items={["Monsters", "Encounters", "Spells", "Settings"]}
/>
<main>
  <div class="flex content-center items-center place-content-center">
    <PageComponent />
  </div>
</main>
