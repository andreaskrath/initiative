<script lang="ts">
  import { Input } from "$components/ui/input/index";
  import { Label } from "$components/ui/label/index";

  let {
    label,
    value = $bindable(),
    type,
    placeholder,
    columns,
    center,
  }: {
    label: string;
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    columns: number;
    center?: boolean;
  } = $props();
</script>

<!-- Have to use inline CSS because dynamic tailwind classes are not picked up by compiler -->

<div class="flex w-full flex-col gap-1.5" style="grid-column: span {columns}">
  <Label class="ml-1"><strong>{label}</strong></Label>
  <Input class={center ? "text-center" : ""} {type} {placeholder} bind:value />
</div>
