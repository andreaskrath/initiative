<script lang="ts">
  import { cn } from "$lib/utils";
  import * as Select from "$components/ui/select/index";
  import { Label } from "$components/ui/label/index";

  let {
    label,
    value = $bindable(),
    placeholder,
    items,
    containerClass,
    labelClass,
    selectClass,
  }: {
    label: string;
    value: string | undefined;
    placeholder: string;
    items: { label: string; value: string }[];
    containerClass?: string;
    labelClass?: string;
    selectClass?: string;
  } = $props();
</script>

<div class={cn("flex w-full flex-col gap-1.5", containerClass)}>
  <Label class={cn("ml-1", labelClass)}>{label}</Label>
  <Select.Root type="single" name={label} bind:value>
    <Select.Trigger class="w-full">
      {value ? value : placeholder}
    </Select.Trigger>
    <Select.Content>
      <Select.Group>
        {#each items as item (item.value)}
          <Select.Item
            value={item.value}
            label={item.label}
            class={cn(selectClass)}
          >
            {item.label}
          </Select.Item>
        {/each}
      </Select.Group>
    </Select.Content>
  </Select.Root>
</div>
