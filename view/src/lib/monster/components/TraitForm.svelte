<script lang="ts">
  import Container from "$components/Container.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import TextArea from "$components/TextArea.svelte";
  import { Button } from "$components/ui/button/index";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import type { FieldErrors } from "$utils/error";

  interface Props {
    trait: { name?: string; description?: string };
    index: number;
    errors?: FieldErrors | null;
    onRemove: () => void;
    showSeparator?: boolean;
    validateCallback?: () => void;
  }

  let {
    trait = $bindable(),
    index,
    errors = null,
    onRemove,
    showSeparator = false,
    validateCallback,
  }: Props = $props();
</script>

<!-- Name -->
<Container class="col-span-9">
  <Label required>Name</Label>
  <Input
    bind:value={trait.name}
    type="text"
    placeholder="Martial Advantage"
    error={errors?.get(`traits.${index}.name`)}
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
    bind:value={trait.description}
    placeholder="Write a description for the trait.."
    error={errors?.get(`traits.${index}.description`)}
    validateCallback={validateCallback}
  />
</Container>

{#if showSeparator}
  <hr class="col-span-10" />
{/if}
