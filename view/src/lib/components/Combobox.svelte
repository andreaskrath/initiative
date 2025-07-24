<script lang="ts">
  import Check from "@lucide/svelte/icons/check";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import * as Command from "$lib/components/ui/command";
  import * as Popover from "$lib/components/ui/popover";
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { tick } from "svelte";

  let {
    value = $bindable(),
    placeholder,
    items,
  }: {
    value?: string;
    placeholder: string;
    items: { label: string; value: string }[];
  } = $props();

  let open = $state(false);
  let inputValue = $state("");

  let selectedItem = $derived(items.find((f) => f.value === value));
  let displayValue = $derived(selectedItem ? selectedItem.label : value);

  let filteredItems = $derived(
    items.filter((item) =>
      item.label.toLowerCase().includes(inputValue.toLowerCase()),
    ),
  );

  let isCustomValue = $derived(
    inputValue &&
      !items.some((f) => f.label.toLowerCase() === inputValue.toLowerCase()),
  );

  function handleSelect(currentValue: string) {
    value = currentValue;
    inputValue = "";
    open = false;
  }

  function handleCustomValue() {
    if (inputValue) {
      value = inputValue;
      inputValue = "";
      open = false;
    }
  }

  async function handleOpenChange(isOpen: boolean) {
    open = isOpen;
    if (isOpen) {
      await tick();
      // Focus the input when popover opens
      const input = document.querySelector("[cmdk-input]") as HTMLInputElement;
      if (input) {
        input.focus();
      }
    }
  }
</script>

<Popover.Root bind:open onOpenChange={handleOpenChange}>
  <Popover.Trigger>
    <Button
      variant="outline"
      role="combobox"
      aria-expanded={open}
      class="w-full justify-between {displayValue
        ? ''
        : 'text-muted-foreground'}"
    >
      {displayValue || placeholder}
      {#if open}
        <ChevronUp class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      {:else}
        <ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      {/if}
    </Button>
  </Popover.Trigger>
  <Popover.Content class="w-full p-0">
    <Command.Root shouldFilter={false}>
      <Command.Input
        placeholder="Search"
        bind:value={inputValue}
        onkeydown={(e) => {
          if (e.key === "Enter" && isCustomValue) {
            e.preventDefault();
            handleCustomValue();
          }
        }}
      />
      <Command.Empty>
        {#if inputValue}
          <div class="py-2 text-center text-sm">
            {inputValue}
          </div>
        {:else}
          No results found.
        {/if}
      </Command.Empty>
      <Command.Group>
        {#if isCustomValue && inputValue}
          <Command.Item
            value={inputValue}
            onSelect={handleCustomValue}
            class="cursor-pointer"
          >
            <Check
              class={cn(
                "mr-2 h-4 w-4",
                value === inputValue ? "opacity-100" : "opacity-0",
              )}
            />
            {inputValue}
          </Command.Item>
          <Command.Separator />
        {/if}

        {#each filteredItems as item}
          <Command.Item
            value={item.value}
            onSelect={() => handleSelect(item.value)}
          >
            <Check
              class={cn(
                "mr-2 h-4 w-4",
                value === item.value ? "opacity-100" : "opacity-0",
              )}
            />
            {item.label}
          </Command.Item>
        {/each}
      </Command.Group>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
