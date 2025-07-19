<script lang="ts">
  import { cn } from "$lib/utils";
  import { Input } from "$components/ui/input/index";

  let {
    value = $bindable(),
    type,
    placeholder,
    class: className,
  }: {
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    class?: string;
  } = $props();
</script>

<Input class={cn("w-full", className)} {type} {placeholder} bind:value />
