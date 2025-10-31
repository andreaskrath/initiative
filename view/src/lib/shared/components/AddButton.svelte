<script lang="ts">
  import CirclePlus from "@lucide/svelte/icons/circle-plus";
  import { Button } from "$components/ui/button/index";
  import type { EventHandler } from "svelte/elements";

  let { onclick }: { onclick?: EventHandler } = $props();
</script>

<Button
  variant="ghost"
  class="text-green-500 hover:text-green-300 dark:text-green-300 dark:hover:text-green-500"
  {onclick}
>
  <CirclePlus />
</Button>
