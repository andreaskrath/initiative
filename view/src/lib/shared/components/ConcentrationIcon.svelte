<script lang="ts">
  let { class: className }: { class?: string } = $props();
  import { cn } from "$utils/utils";
</script>

<span
  class={cn(
    "ml-2 flex h-5 w-5 items-center justify-center rounded-xs font-bold",
    "bg-black text-white",
    "dark:bg-white dark:text-black",
    className,
  )}
>
  C
</span>
