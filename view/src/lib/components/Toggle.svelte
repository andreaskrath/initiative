<script lang="ts">
  import { cn } from "$lib/utils";
  import { Toggle } from "$components/ui/toggle/index";
  import type { Snippet } from "svelte";

  let {
    checked = $bindable(),
    class: className,
    children,
  }: {
    checked: boolean;
    class?: string;
    children: Snippet;
  } = $props();
</script>

<!-- <div class={cn("flex items-center space-x-2", containerClass)}> -->
<Toggle class={cn("w-full", className)} bind:pressed={checked}>
  {@render children?.()}
</Toggle>
<!-- </div> -->
