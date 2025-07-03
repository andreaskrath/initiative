<script lang="ts">
  let {
    label,
    value = $bindable(),
    type,
    placeholder,
    labelSize,
    inputSize,
    center = false,
  }: {
    label: string;
    value: string | number | null;
    type: "text" | "number";
    placeholder: string;
    labelSize: number;
    inputSize: number;
    center?: boolean;
  } = $props();
</script>

<!-- Have to use inline CSS because dynamic tailwind classes are not picked up by compiler -->

<div class="ig-cell preset-tonal" style="grid-column: span {labelSize}">
  {label}
</div>
<input
  class="ig-input {type === 'number' ? 'text-center no-spinner' : ''} {center
    ? 'text-center'
    : ''}"
  style="grid-column: span {inputSize}"
  {type}
  {placeholder}
  bind:value
/>
