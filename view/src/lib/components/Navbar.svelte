<script lang="ts">
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

<div class="preset-filled-surface-100-900 flex justify-center">
  <div class="grid grid-cols-4 gap-4 py-2 h-[50px] w-[1200px]">
    {#each items as item, index}
      <button
        type="button"
        class="btn text-center col-span-1 {index === currentItem
          ? 'preset-glass-secondary preset-outlined'
          : 'hover:preset-filled-surface-100-900'}"
        onclick={(e) => updateItem(e, index)}>{item}</button
      >
    {/each}
  </div>
</div>
<hr class="hr" />

<style>
  .preset-glass-secondary {
    background: color-mix(
      in oklab,
      var(--color-secondary-500) 40%,
      transparent
    );
    box-shadow: 0 0px 30px
      color-mix(in oklab, var(--color-secondary-500) 50%, transparent) inset;
    backdrop-filter: blur(16px);
  }
</style>
