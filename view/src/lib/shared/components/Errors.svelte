<script lang="ts">
  import * as Alert from "$components/ui/alert/index";
  import CircleAlert from "@lucide/svelte/icons/circle-alert";

  let { title, errors = $bindable() }: { title: string; errors: string[] } =
    $props();
</script>

{#if errors.length > 0}
  <Alert.Root variant="destructive">
    <CircleAlert />
    <Alert.Title>{title}</Alert.Title>
    <Alert.Description>
      <p>The following issues were identified.</p>
      <ul class="list-inside list-disc text-sm">
        {#each errors as error}
          <li>{error}</li>
        {/each}
      </ul>
    </Alert.Description>
  </Alert.Root>
{/if}
