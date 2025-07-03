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

<div class="input-group grid-cols-{columns} border-none outline-none py-2 px-2">
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
