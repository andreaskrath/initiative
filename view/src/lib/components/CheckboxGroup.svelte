<script lang="ts">
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
    <label
      class="flex items-center space-x-2 space-y-2 border-none outline-none col-span-1"
    >
      <input
        class="checkbox"
        type="checkbox"
        bind:checked={checkedItems[item]}
      />
      <p>{item}</p>
    </label>
  {/each}
</div>
