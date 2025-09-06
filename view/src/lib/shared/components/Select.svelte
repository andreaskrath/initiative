<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import * as Select from "$components/ui/select/index";
  import Error from "$components/Error.svelte";
  import type { FocusEventHandler } from "svelte/elements";

  let {
    value = $bindable(),
    placeholder,
    items,
    error = "",
    validateCallback,
    class: className,
  }: {
    value: string | undefined;
    placeholder: string;
    items: { label: string; value: string }[];
    error?: string;
    validateCallback?: FocusEventHandler<HTMLButtonElement>;
    class?: string;
  } = $props();

  const triggerContent = $derived(
    items.find((item) => item.value === value)?.label ?? placeholder,
  );
</script>

<div class="relative">
  <Select.Root type="single" bind:value>
    <Select.Trigger
      onfocusout={validateCallback}
      class="w-full truncate {error
        ? '!ring-destructive/20 !dark:ring-destructive/40 !border-destructive'
        : ''}"
    >
      {triggerContent}
    </Select.Trigger>
    <Select.Content>
      <Select.Group>
        {#each items as item (item.value)}
          <Select.Item
            value={item.value}
            label={item.label}
            class={cn(className)}
          >
            {item.label}
          </Select.Item>
        {/each}
      </Select.Group>
    </Select.Content>
  </Select.Root>

  {#if error}
    <Error {error} />
  {/if}
</div>
