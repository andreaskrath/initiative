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

<!-- Have to use inline CSS because dynamic tailwind classes are not picked up by compiler -->

<div class="ig-cell preset-tonal" style="grid-column: span {labelSize}">
  {title}
</div>
<select
  class="ig-select text-center"
  style="grid-column: span {inputSize}"
  bind:value
>
  {#each items as item}
    <option>{item}</option>
  {/each}
</select>
