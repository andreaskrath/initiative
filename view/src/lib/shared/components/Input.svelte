<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Input } from "$components/ui/input/index";
  import Error from "./Error.svelte";

  let {
    value = $bindable(),
    type,
    placeholder,
    error = "",
    class: className,
  }: {
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    error?: string;
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
  />
  {#if error}
    <Error {error} />
  {/if}
</div>
