<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";

  let {
    title,
    onClick,
  }: { title: string; onClick: MouseEventHandler<HTMLButtonElement> } =
    $props();
</script>

<button type="button" class="btn preset-filled-secondary-500" onclick={onClick}
  >{title}</button
>
