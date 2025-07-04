<script lang="ts">
  import Checkbox from "$components/Checkbox.svelte";
  let {
    items,
    checkedItems = $bindable(),
    columns,
  }: {
    items: string[];
    checkedItems: Record<string, boolean>;
    columns: number;
  } = $props();
</script>

<!-- Have to use inline CSS because dynamic tailwind classes are not picked up by compiler -->

<div
  class="input-group border-none outline-none py-2 px-2"
  style="grid-template-columns: repeat({columns}, minmax(0, 1fr))"
>
  {#each items as item}
    <Checkbox label={item} bind:value={checkedItems[item]} />
  {/each}
</div>
