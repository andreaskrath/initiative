<script lang="ts">
  import "./app.css";
  import Monsters from "./lib/Monsters.svelte";
  import Encounters from "./lib/Encounters.svelte";
  import NotFound from "./lib/NotFound.svelte";
  import Navbar from "./lib/components/Navbar.svelte";

  const pages = [Monsters, Encounters];
  let currentPage: number = $state(0);
  const PageComponent = $derived(pages[currentPage] || NotFound);
</script>

<main>
  <Navbar bind:item={currentPage} />
  <div class="flex content-center items-center place-content-center">
    <PageComponent />
  </div>
</main>
