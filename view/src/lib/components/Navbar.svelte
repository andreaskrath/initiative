<script lang="ts">
  let { item = $bindable() }: { item: number } = $props();

  function updateItem(newItem: number) {
    return (event: MouseEvent) => {
      event.preventDefault();
      item = newItem;
    };
  }
</script>

<ul id="navbar">
  <li>
    <a class="title navbar-item" href="/" onclick={updateItem(0)}>Monsters</a>
  </li>
  <li>
    <a class="title navbar-item" href="/" onclick={updateItem(1)}>Encounters</a>
  </li>
</ul>

<style>
  ul#navbar li {
    display: inline;
  }
</style>
