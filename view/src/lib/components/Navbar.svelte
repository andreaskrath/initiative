<script lang="ts">
  import { Navigation } from "@skeletonlabs/skeleton-svelte";
  import Swords from "@lucide/svelte/icons/swords";
  import Settings from "@lucide/svelte/icons/settings";
  import Panda from "@lucide/svelte/icons/panda";
  import type { Component } from "svelte";

  let { item = $bindable() }: { item: number } = $props();

  const navbarItems: { title: string; icon: Component }[] = [
    {
      title: "Monsters",
      icon: Panda,
    },
    {
      title: "Encounters",
      icon: Swords,
    },
    {
      title: "Settings",
      icon: Settings,
    },
  ];

  function updateItem(id: string) {
    item = navbarItems.findIndex((navItem) => navItem.title === id);
  }
</script>

<div class="preset-filled-surface-100-900 flex justify-center">
  <div class="card flex gap-3 p-3 h-[100px] w-[1200px]">
    {#each navbarItems as navbarItem, index}
      <Navigation.Tile
        label={navbarItem.title}
        selected={item === index}
        id={navbarItem.title}
        onclick={updateItem}><navbarItem.icon size={48} /></Navigation.Tile
      >
    {/each}
  </div>
</div>
