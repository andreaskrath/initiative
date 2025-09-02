<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import * as Select from "$components/ui/select/index";
  import Error from "$components/Error.svelte";

  let {
    value = $bindable(),
    placeholder,
    items,
    error = "",
    class: className,
  }: {
    value: string | undefined;
    placeholder: string;
    items: { label: string; value: string }[];
    error?: string;
    class?: string;
  } = $props();

  const triggerContent = $derived(
    items.find((item) => item.value === value)?.label ?? placeholder,
  );
</script>

<div class="relative">
  <Select.Root type="single" bind:value>
    <Select.Trigger class="w-full truncate {error ? 'border-red-500' : ''}">
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
