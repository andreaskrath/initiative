<script lang="ts">
  import { cn } from "$shared/utils/utils";
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

<Toggle class={cn("w-full", className)} bind:pressed={checked}>
  {@render children?.()}
</Toggle>
