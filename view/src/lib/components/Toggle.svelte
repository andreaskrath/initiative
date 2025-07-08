<script lang="ts">
  import { cn } from "$lib/utils";
  import { Toggle } from "$components/ui/toggle/index";

  let {
    label,
    checked = $bindable(),
    containerClass,
    toggleClass,
  }: {
    label: string;
    checked: boolean;
    containerClass?: string;
    toggleClass?: string;
  } = $props();
</script>

<div class={cn("flex items-center space-x-2", containerClass)}>
  <Toggle class={cn("w-full", toggleClass)} bind:pressed={checked}>
    {label}
  </Toggle>
</div>
