<script lang="ts">
  let {
    title,
    value = $bindable(),
    items,
    labelSize,
    inputSize,
  }: {
    title: string;
    value: string | null;
    items: string[];
    labelSize: number;
    inputSize: number;
  } = $props();
</script>

<div class="ig-cell preset-tonal col-span-{labelSize}">{title}</div>
<select class="ig-select col-span-{inputSize} text-center" bind:value>
  {#each items as item}
    <option>{item}</option>
  {/each}
</select>
