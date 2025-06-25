<script lang="ts">
  import type { MouseEventHandler } from "svelte/elements";

  let {
    title,
    onClick,
  }: { title: string; onClick: MouseEventHandler<HTMLButtonElement> } =
    $props();
</script>

<button
  onclick={onClick}
  class="button clickable border border-2 border-gray-800 bg-gray-500 dnd-font text-2xl px-2 py-1 cursor-pointer active:scale-95 transition-transform duration-100"
>
  {title}
</button>

<style>
  .button {
    text-shadow:
      -0.5px -0.5px 0 #000,
      0.5px -0.5px 0 #000,
      -0.5px 0.5px 0 #000,
      0.5px 0.5px 0 #000;
  }
</style>
