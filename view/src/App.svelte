<script lang="ts">
  import "./app.css";
  import Monsters from "./lib/monsters/Monsters.svelte";
  import Encounters from "./lib/encounters/Encounters.svelte";
  import NotFound from "./lib/NotFound.svelte";
  import Settings from "./lib/settings/Settings.svelte";
  import Navbar from "./lib/components/Navbar.svelte";
  import type { Component } from "svelte";

  const pages: Component[] = [Monsters, Encounters, Settings];
  let currentPage: number = $state(0);
  const PageComponent = $derived(pages[currentPage] || NotFound);
</script>

<Navbar bind:item={currentPage} />
<main>
  <div class="flex content-center items-center place-content-center">
    <PageComponent />
  </div>
</main>
