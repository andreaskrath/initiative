<script lang="ts">
  import { Toggle } from "$components/ui/toggle/index";

  let {
    label,
    checked = $bindable(),
    columns,
    center,
  }: {
    label: string;
    checked: boolean;
    columns: number;
    center?: boolean;
  } = $props();
</script>

<div
  class="flex items-center space-x-2
  {center ? 'justify-center' : ''}"
  style="grid-column: span {columns}"
>
  <Toggle class="w-full" bind:pressed={checked}>
    {label}
  </Toggle>
</div>
