<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "$lib/utils";

  let {
    variant,
    class: className,
    children,
  }: {
    variant: "default" | "muted";
    class?: string;
    children?: Snippet;
  } = $props();

  const getVariantClass = (variantType: string) => {
    switch (variantType) {
      case "default":
        return "";
      case "muted":
        return "text-muted-foreground";
      default:
        return "";
    }
  };

  let variantClass = getVariantClass(variant);
</script>

<h2
  class={cn("text-muted-foreground mb-2 ml-1 text-xl", className, variantClass)}
>
  {@render children?.()}
</h2>
