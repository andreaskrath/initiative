<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Textarea } from "$components/ui/textarea/index";

  let {
    value = $bindable(),
    placeholder,
    class: className,
  }: {
    value?: string;
    placeholder: string;
    class?: string;
  } = $props();
</script>

<Textarea bind:value {placeholder} class={cn("w-full", className)} />
