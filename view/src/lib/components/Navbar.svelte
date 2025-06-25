<script lang="ts">
  let { item = $bindable() }: { item: number } = $props();

  const navbarItems = ["Monsters", "Encounters"];

  function updateItem(newItem: number) {
    return (event: MouseEvent) => {
      event.preventDefault();
      item = newItem;
    };
  }
</script>

<div class="flex justify-center bg-stone-700 py-4 border-b border-amber-200">
  <ul id="navbar" class="flex gap-8">
    {#each navbarItems as title, index}
      <li>
        <a
          class="navbar-item dnd-font text-3xl {index === item
            ? 'text-amber-300'
            : 'clickable'}"
          href="/"
          onclick={updateItem(index)}>{title}</a
        >
      </li>
    {/each}
  </ul>
</div>

<style>
  ul#navbar li {
    display: inline;
  }

  .navbar-item {
    text-shadow:
      -0.5px -0.5px 0 #000,
      0.5px -0.5px 0 #000,
      -0.5px 0.5px 0 #000,
      0.5px 0.5px 0 #000;
  }
</style>
