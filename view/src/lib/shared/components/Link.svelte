<script lang="ts">
  import { route, RouteOptions } from "@mateothegreat/svelte5-router";
  import type { Snippet } from "svelte";
  import { cn } from "$shared/utils/utils";

  let {
    options,
    href,
    label,
    class: className,
    children,
  }: {
    options?: RouteOptions;
    href: string;
    label: string;
    class?: string;
    children?: Snippet;
  } = $props();

  if (!options) {
    options = new RouteOptions();
  }
</script>

<a
  use:route={options}
  {href}
  class={cn(
    "hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground block space-y-1 rounded-md p-3 leading-none no-underline transition-colors outline-none select-none",
    className,
  )}
>
  <div class="text-sm leading-none font-medium">{label}</div>
  {@render children?.()}
</a>
