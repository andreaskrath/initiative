<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Textarea } from "$components/ui/textarea/index";
  import Error from "$components/Error.svelte";
  import type { FocusEventHandler } from "svelte/elements";

  let {
    value = $bindable(),
    placeholder,
    error = "",
    validateCallback,
    class: className,
  }: {
    value?: string;
    placeholder: string;
    error?: string;
    validateCallback?: FocusEventHandler<HTMLTextAreaElement>;
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
    onfocusout={validateCallback}
    rows={5}
  />

  {#if error}
    <Error {error} />
  {/if}
</div>
