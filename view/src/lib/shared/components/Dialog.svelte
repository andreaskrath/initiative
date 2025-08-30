<script lang="ts">
  import * as Dialog from "$components/ui/dialog/index";
  import type { Snippet } from "svelte";

  let {
    title,
    description,
    trigger,
    content,
    footer,
    open = $bindable(),
  }: {
    title: string;
    description: string;
    trigger: Snippet;
    content: Snippet;
    footer: Snippet;
    open: boolean;
  } = $props();
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger>
    {@render trigger()}
  </Dialog.Trigger>
  <Dialog.Content class="w-[800px] !max-w-none">
    <Dialog.Header>
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description>
        {description}
      </Dialog.Description>
    </Dialog.Header>
    {@render content()}
    <Dialog.Footer>
      {@render footer()}
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
