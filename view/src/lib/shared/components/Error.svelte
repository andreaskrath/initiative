<script lang="ts">
  import CircleAlert from "@lucide/svelte/icons/circle-alert";
  import * as Tooltip from "$components/ui/tooltip/index";

  let { error }: { error: string } = $props();
</script>

<Tooltip.Provider>
  <Tooltip.Root delayDuration={200}>
    <Tooltip.Trigger class="absolute right-1 bottom-10 text-red-500">
      <CircleAlert class="h-4 w-4" />
    </Tooltip.Trigger>
    <Tooltip.Content side="top" align="center">
      <p>{error}</p>
    </Tooltip.Content>
  </Tooltip.Root>
</Tooltip.Provider>
