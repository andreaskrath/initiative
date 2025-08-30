<script lang="ts">
  import Link from "$components/Link.svelte";
  import { ModeWatcher } from "mode-watcher";
  import ModeToggle from "$components/ModeToggle.svelte";
  import * as NavigationMenu from "$components/ui/navigation-menu/index.js";
  import { navigationMenuTriggerStyle } from "$components/ui/navigation-menu/navigation-menu-trigger.svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type ListItemProps = HTMLAttributes<HTMLAnchorElement> & {
    label: string;
    href: string;
    content: string;
  };
</script>

{#snippet ListItem({ label, content, href }: ListItemProps)}
  <li>
    <NavigationMenu.Link>
      {#snippet child()}
        <Link {href} {label}>
          <p class="text-muted-foreground line-clamp-2 text-sm leading-snug">
            {content}
          </p>
        </Link>
      {/snippet}
    </NavigationMenu.Link>
  </li>
{/snippet}

<ModeWatcher />
<div
  class="relative mx-auto flex h-[50px] w-[1000px] max-w-[1000px] items-center"
>
  <div class="absolute left-4">
    <Link href="/" label="">
      <img
        src="/images/logo.svg"
        alt="Application logo"
        class="h-[40px] w-auto"
      />
    </Link>
  </div>
  <NavigationMenu.Root viewport={false} class="z-50 mx-auto mt-2 mb-2">
    <NavigationMenu.List>
      <!-- Monsters -->
      <NavigationMenu.Item>
        <NavigationMenu.Trigger>Monsters</NavigationMenu.Trigger>
        <NavigationMenu.Content>
          <ul class="w-[400px] p-2">
            {@render ListItem({
              href: "/monsters",
              label: "Monster List",
              content: "A list of existing monsters.",
            })}
            {@render ListItem({
              href: "/monsters/create",
              label: "Monster Builder",
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
              label: "Encounter List",
              content: "A list of existing encounters.",
            })}
            {@render ListItem({
              href: "/encounters/create",
              label: "Encounter Builder",
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
              label: "Spell List",
              content: "A list of existing spells.",
            })}
            {@render ListItem({
              href: "/spells/create",
              label: "Spell Builder",
              content: "Build a new spell.",
            })}
          </ul>
        </NavigationMenu.Content>
      </NavigationMenu.Item>

      <!-- Settings -->
      <NavigationMenu.Item>
        <NavigationMenu.Link>
          {#snippet child()}
            <Link
              href="/settings"
              label="Settings"
              class={navigationMenuTriggerStyle()}
            />
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
