<script lang="ts">
  import { cn } from "$lib/utils";
  import { Label } from "$components/ui/label/index";
  import { Textarea } from "$components/ui/textarea/index";

  let {
    label,
    value = $bindable(),
    placeholder,
    id,
    containerClass,
    labelClass,
    textareaClass,
  }: {
    label: string;
    value?: string;
    placeholder: string;
    id: string;
    containerClass?: string;
    labelClass?: string;
    textareaClass?: string;
  } = $props();
</script>

<div class={cn("flex w-full flex-col gap-1.5", containerClass)}>
  <Label class={cn("ml-1", labelClass)} for={id}>{label}</Label>
  <Textarea bind:value {placeholder} {id} class={cn("w-full", textareaClass)} />
</div>
