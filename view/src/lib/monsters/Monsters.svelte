<script lang="ts">
  import Button from "$components/Button.svelte";
  import CreateMonster from "./CreateMonster.svelte";
  import NotFound from "../NotFound.svelte";
  import ViewMonster from "./ViewMonster.svelte";
  import type { Component } from "svelte";

  const pages: Component[] = [ViewMonster, CreateMonster];
  let currentPage: number = $state(0);
  const PageComponent = $derived(pages[currentPage] || NotFound);

  function updatePage(newPage: number) {
    return function (event: MouseEvent) {
      event.preventDefault();
      currentPage = newPage;
    };
  }
</script>

<div>
  <PageComponent />

  <!-- Only render "Create New Monster" button if not already on that page -->
  {#if currentPage !== 1}
    <Button title="Create New Monster" onClick={updatePage(1)} />
  {/if}
</div>
