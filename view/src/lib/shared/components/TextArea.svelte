<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Textarea } from "$components/ui/textarea/index";
  import Error from "$components/Error.svelte";

  let {
    value = $bindable(),
    placeholder,
    error = "",
    class: className,
  }: {
    value?: string;
    placeholder: string;
    error?: string;
    class?: string;
  } = $props();

  let errorClasses = $derived(
    error
      ? "!ring-destructive/20 !dark:ring-destructive/40 !border-destructive"
      : "",
  );
</script>

<div class="relative w-full">
  <Textarea
    bind:value
    {placeholder}
    class={cn("w-full", className, errorClasses)}
    rows={5}
  />

  {#if error}
    <Error {error} />
  {/if}
</div>
