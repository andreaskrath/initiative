<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Label } from "$components/ui/label";
  import type { Snippet } from "svelte";
  import Required from "./Required.svelte";

  let {
    class: className,
    children,
    required = false,
  }: {
    class?: string;
    children: Snippet;
    required?: boolean;
  } = $props();
</script>

<Label class={cn("relative ml-1 inline-block w-fit", className)}>
  {@render children?.()}
  {#if required}
    <Required />
  {/if}
</Label>
