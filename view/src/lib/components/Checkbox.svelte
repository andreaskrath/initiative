<script lang="ts">
  let {
    label,
    value = $bindable(),
    columns,
    center,
  }: {
    label: string;
    value: boolean;
    columns?: number;
    center?: boolean;
  } = $props();
</script>

<div
  class="flex items-center space-x-2 border-none outline-none
  {columns ? '' : 'col-span-1'} 
  {center ? 'justify-center' : ''}"
  style={columns ? "grid-column: span " + columns : ""}
>
  <input class="checkbox mb-0.5" type="checkbox" bind:checked={value} />
  <p>{label}</p>
</div>
