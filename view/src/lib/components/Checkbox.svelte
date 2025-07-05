<script lang="ts">
  import { Checkbox } from "$components/ui/checkbox/index";
  import { Label } from "$components/ui/label/index";

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
  <Checkbox id={label} bind:checked />
  <Label for={label} class="text-base">{label}</Label>
</div>
