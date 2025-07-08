<script lang="ts">
  import { cn } from "$lib/utils";
  import { Input } from "$components/ui/input/index";
  import { Label } from "$components/ui/label/index";

  let {
    label,
    value = $bindable(),
    type,
    placeholder,
    containerClass,
    labelClass,
    inputClass,
  }: {
    label: string;
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    containerClass?: string;
    labelClass?: string;
    inputClass?: string;
  } = $props();
</script>

<div class={cn("flex w-full flex-col gap-1.5", containerClass)}>
  <Label class={cn("ml-1", labelClass)}>{label}</Label>
  <Input class={cn(inputClass)} {type} {placeholder} bind:value />
</div>
