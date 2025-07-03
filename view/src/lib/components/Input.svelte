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

<div class="ig-cell preset-tonal col-span-{labelSize}">{label}</div>
<input
  class="ig-input col-span-{inputSize} {type === 'number'
    ? 'text-center no-spinner'
    : ''} {center ? 'text-center' : ''}"
  {type}
  {placeholder}
  bind:value
/>
