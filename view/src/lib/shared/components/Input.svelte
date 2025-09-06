<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Input } from "$components/ui/input/index";
  import Error from "./Error.svelte";
  import type { FocusEventHandler } from "svelte/elements";

  let {
    value = $bindable(),
    type,
    placeholder,
    error = "",
    validateCallback,
    class: className,
  }: {
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    error?: string;
    validateCallback?: FocusEventHandler<HTMLInputElement>;
    class?: string;
  } = $props();
</script>

<div class="relative">
  <Input
    class={cn("w-full", className)}
    {type}
    aria-invalid={error !== ""}
    {placeholder}
    bind:value
    onfocusout={validateCallback}
  />
  {#if error}
    <Error {error} />
  {/if}
</div>
