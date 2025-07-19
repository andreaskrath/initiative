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
  class="input-group border-none px-2 py-2 outline-none"
  style="grid-template-columns: repeat({columns}, minmax(0, 1fr))"
>
  {#each items as item}
    <Checkbox label={item} bind:checked={checkedItems[item]} columns={1} />
  {/each}
</div>
