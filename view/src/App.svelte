<script lang="ts">
  import "./app.css";

  import {
    Router,
    type RouteConfig,
    StatusCode,
    type RouteResult,
  } from "@mateothegreat/svelte5-router";

  import Home from "$routes/Home.svelte";
  import NotFound from "$routes/NotFound.svelte";
  import Settings from "$routes/Settings.svelte";
  import MonsterList from "$routes/MonsterList.svelte";
  import MonsterBuilder from "$routes/MonsterBuilder.svelte";
  import EncounterList from "$routes/EncounterList.svelte";
  import EncounterBuilder from "$routes/EncounterBuilder.svelte";
  import SpellList from "$routes/SpellList.svelte";
  import SpellBuilder from "$routes/SpellBuilder.svelte";

  import Navbar from "$components/Navbar.svelte";
  import { Toaster } from "$lib/components/ui/sonner";

  const routes: RouteConfig[] = [
    {
      component: Home,
    },
    {
      path: "monsters",
      component: MonsterList,
    },
    {
      path: "monsters/create",
      component: MonsterBuilder,
    },
    {
      path: "encounters",
      component: EncounterList,
    },
    {
      path: "encounters/create",
      component: EncounterBuilder,
    },
    {
      path: "spells",
      component: SpellList,
    },
    {
      path: "spells/create",
      component: SpellBuilder,
    },
    {
      path: "settings",
      component: Settings,
    },
  ];
</script>

<Toaster duration={5000} position="bottom-right" />
<Navbar />
<main>
  <div class="mx-auto w-[1200px] py-2">
    <Router
      {routes}
      statuses={{
        [StatusCode.NotFound]: (_: RouteResult) => {
          return {
            component: NotFound,
          };
        },
      }}
    />
  </div>
</main>
