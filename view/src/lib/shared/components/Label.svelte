<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Label } from "$components/ui/label";
  import type { Snippet } from "svelte";

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

<Label class={cn("ml-1", className)}>
  {@render children?.()}
  {#if required}
    <span class="text-sm text-red-500">*</span>
  {/if}
</Label>
