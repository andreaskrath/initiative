<script lang="ts">
  import { ModeWatcher } from "mode-watcher";
  import ModeToggle from "$components/ModeToggle.svelte";
  import * as NavigationMenu from "$lib/components/ui/navigation-menu/index.js";
  import { navigationMenuTriggerStyle } from "$lib/components/ui/navigation-menu/navigation-menu-trigger.svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type ListItemProps = HTMLAttributes<HTMLAnchorElement> & {
    title: string;
    href: string;
    content: string;
  };
</script>

{#snippet ListItem({ title, content, href }: ListItemProps)}
  <li>
    <NavigationMenu.Link>
      {#snippet child()}
        <a
          {href}
          class="hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground block space-y-1 rounded-md p-3 leading-none no-underline transition-colors outline-none select-none"
        >
          <div class="text-sm leading-none font-medium">{title}</div>
          <p class="text-muted-foreground line-clamp-2 text-sm leading-snug">
            {content}
          </p>
        </a>
      {/snippet}
    </NavigationMenu.Link>
  </li>
{/snippet}

<ModeWatcher />
<div class="relative w-full">
  <NavigationMenu.Root viewport={false} class="mx-auto mt-2 mb-2">
    <NavigationMenu.List>
      <!-- Home -->
      <NavigationMenu.Item>
        <NavigationMenu.Link>
          {#snippet child()}
            <a href="/" class={navigationMenuTriggerStyle()}>Home</a>
          {/snippet}
        </NavigationMenu.Link>
      </NavigationMenu.Item>

      <!-- Monsters -->
      <NavigationMenu.Item>
        <NavigationMenu.Trigger>Monsters</NavigationMenu.Trigger>
        <NavigationMenu.Content>
          <ul class="w-[400px] p-2">
            {@render ListItem({
              href: "/monsters",
              title: "Monster List",
              content: "A list of existing monsters.",
            })}
            {@render ListItem({
              href: "/monsters/create",
              title: "Monster Builder",
              content: "Build a new monster.",
            })}
          </ul>
        </NavigationMenu.Content>
      </NavigationMenu.Item>

      <!-- Encounters -->
      <NavigationMenu.Item>
        <NavigationMenu.Trigger>Encounters</NavigationMenu.Trigger>
        <NavigationMenu.Content>
          <ul class="w-[400px] p-2">
            {@render ListItem({
              href: "/encounters",
              title: "Encounter List",
              content: "A list of existing encounters.",
            })}
            {@render ListItem({
              href: "/encounters/create",
              title: "Encounter Builder",
              content: "Build a new encounter.",
            })}
          </ul>
        </NavigationMenu.Content>
      </NavigationMenu.Item>

      <!-- Spells -->
      <NavigationMenu.Item>
        <NavigationMenu.Trigger>Spells</NavigationMenu.Trigger>
        <NavigationMenu.Content>
          <ul class="w-[400px] p-2">
            {@render ListItem({
              href: "/spells",
              title: "Spell List",
              content: "A list of existing spells.",
            })}
            {@render ListItem({
              href: "/spells/create",
              title: "Spell Builder",
              content: "Build a new spell.",
            })}
          </ul>
        </NavigationMenu.Content>
      </NavigationMenu.Item>

      <!-- Settings -->
      <NavigationMenu.Item>
        <NavigationMenu.Link>
          {#snippet child()}
            <a href="/settings" class={navigationMenuTriggerStyle()}>Settings</a
            >
          {/snippet}
        </NavigationMenu.Link>
      </NavigationMenu.Item>
    </NavigationMenu.List>
  </NavigationMenu.Root>
  <div class="absolute top-1/2 right-4 -translate-y-1/2">
    <ModeToggle />
  </div>
</div>
<hr />
