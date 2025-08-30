<script lang="ts">
  import { cn } from "$shared/utils/utils";
  import { Input } from "$components/ui/input/index";

  let {
    value = $bindable(),
    type,
    placeholder,
    error = false,
    class: className,
  }: {
    value: string | number | undefined;
    type: "text" | "number";
    placeholder: string;
    error?: boolean;
    class?: string;
  } = $props();
</script>

<Input
  class={cn("w-full", className)}
  {type}
  aria-invalid={error}
  {placeholder}
  bind:value
/>
