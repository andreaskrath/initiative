<script lang="ts">
  import type { CreatureReminder, Trigger } from "$types";
  import Container from "$components/Container.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import Select from "$components/Select.svelte";
  import TextArea from "$components/TextArea.svelte";
  import { Button } from "$components/ui/button/index";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import type { FieldErrors } from "$utils/error";

  interface Props {
    reminder: CreatureReminder;
    index: number;
    triggerOptions: Array<{ value: Trigger; label: string }>;
    errors?: FieldErrors | null;
    onRemove: () => void;
    showSeparator?: boolean;
    validateCallback?: () => void;
  }

  let {
    reminder = $bindable(),
    index,
    triggerOptions,
    errors = null,
    onRemove,
    showSeparator = false,
    validateCallback,
  }: Props = $props();
</script>

<!-- Name -->
<Container class="col-span-4">
  <Label required>Name</Label>
  <Input
    bind:value={reminder.name}
    type="text"
    placeholder="Martial Advantage"
    error={errors?.get(`reminders.${index}.name`)}
    validateCallback={validateCallback}
  />
</Container>

<!-- Trigger Type -->
<Container class="col-span-5">
  <Label required>Trigger</Label>
  <Select
    bind:value={reminder.trigger}
    placeholder="Select a trigger type"
    items={triggerOptions}
    error={errors?.get(`reminders.${index}.trigger`)}
    validateCallback={validateCallback}
  />
</Container>

<!-- Remove Button -->
<div class="col-span-1 col-start-10 flex justify-center">
  <Button
    variant="ghost"
    size="icon"
    class="mt-5 w-full text-red-300 hover:text-red-600"
    onclick={onRemove}
  >
    <CircleX />
  </Button>
</div>

<!-- Description -->
<Container class="col-span-10">
  <Label required>Description</Label>
  <TextArea
    bind:value={reminder.description}
    placeholder="Write a description for the reminder.."
    error={errors?.get(`reminders.${index}.description`)}
    validateCallback={validateCallback}
  />
</Container>

{#if showSeparator}
  <hr class="col-span-10" />
{/if}
