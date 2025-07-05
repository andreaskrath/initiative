<script lang="ts">
  import * as Select from "$components/ui/select/index";
  import { Label } from "$components/ui/label/index";

  let {
    label,
    value = $bindable(),
    placeholder,
    items,
    columns,
  }: {
    label: string;
    value: string | undefined;
    placeholder: string;
    items: { label: string; value: string }[];
    columns: number;
  } = $props();
</script>

<!-- Have to use inline CSS because dynamic tailwind classes are not picked up by compiler -->

<div
  class="col-span-3 flex w-full flex-col gap-1.5"
  style="grid-column: span {columns}"
>
  <Label class="ml-1"><strong>{label}</strong></Label>
  <Select.Root type="single" name={label} bind:value>
    <Select.Trigger class="w-full">
      {value ? value : placeholder}
    </Select.Trigger>
    <Select.Content>
      <Select.Group>
        {#each items as item (item.value)}
          <Select.Item value={item.value} label={item.label}>
            {item.label}
          </Select.Item>
        {/each}
      </Select.Group>
    </Select.Content>
  </Select.Root>
</div>
