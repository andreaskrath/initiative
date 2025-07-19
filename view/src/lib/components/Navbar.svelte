<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import ModeToggle from "$components/ModeToggle.svelte";

  let {
    currentItem = $bindable(),
    items,
  }: {
    currentItem: number;
    items: string[];
  } = $props();

  function updateItem(event: MouseEvent, index: number) {
    event.preventDefault();
    currentItem = index;
  }
</script>

<ModeWatcher />
<div class="relative w-full">
  <div class="mx-auto grid h-[50px] w-[1200px] grid-cols-4 gap-4 py-2">
    {#each items as item, index}
      <button
        type="button"
        class="btn col-span-1 text-center"
        onclick={(e) => updateItem(e, index)}>{item}</button
      >
    {/each}
  </div>
  <div class="absolute top-1/2 right-4 -translate-y-1/2">
    <ModeToggle />
  </div>
</div>
<hr class="hr" />
