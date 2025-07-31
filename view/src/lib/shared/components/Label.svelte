<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Label } from "$components/ui/label";
  import type { Snippet } from "svelte";

  let {
    class: className,
    children,
  }: {
    class?: string;
    children: Snippet;
  } = $props();
</script>

<Label class={cn("ml-1", className)}>
  {@render children?.()}
</Label>
