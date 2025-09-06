<script lang="ts">
  import Check from "@lucide/svelte/icons/check";
  import ChevronUp from "@lucide/svelte/icons/chevron-up";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import * as Command from "$components/ui/command";
  import * as Popover from "$components/ui/popover";
  import { Button } from "$components/ui/button";
  import { cn } from "$shared/utils/utils";
  import { tick } from "svelte";
  import Error from "$components/Error.svelte";

  let {
    value = $bindable(),
    placeholder,
    items,
    error = "",
  }: {
    value?: string;
    placeholder: string;
    items: { label: string; value: string }[];
    error?: string;
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
  let displayClasses = $derived(displayValue ? "" : "text-muted-foreground");
  let errorClasses = $derived(
    error
      ? "!ring-destructive/20 !dark:ring-destructive/40 !border-destructive"
      : "",
  );

  const handleSelect = (currentValue: string) => {
    value = currentValue;
    inputValue = "";
    open = false;
  };

  const handleCustomValue = () => {
    if (inputValue) {
      value = inputValue;
      inputValue = "";
      open = false;
    }
  };

  const handleOpenChange = async (isOpen: boolean) => {
    open = isOpen;
    if (isOpen) {
      await tick();

      const input = document.querySelector("[cmdk-input]") as HTMLInputElement;
      if (input) {
        input.focus();
      }
    }
  };
</script>

<Popover.Root bind:open onOpenChange={handleOpenChange}>
  <Popover.Trigger>
    <div class="relative w-full">
      <Button
        variant="outline"
        role="combobox"
        aria-expanded={open}
        class={cn(
          "w-full justify-between truncate",
          displayClasses,
          errorClasses,
        )}
      >
        {displayValue || placeholder}
        {#if open}
          <ChevronUp class="ml-2 h-4 w-4 shrink-0 opacity-50" />
        {:else}
          <ChevronDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
        {/if}
      </Button>
      {#if error}
        <Error {error} />
      {/if}
    </div>
  </Popover.Trigger>
  <Popover.Content class="mt-1 p-0">
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
