<script lang="ts">
  import "./app.css";

  import {
    Router,
    type RouteConfig,
    StatusCode,
    type RouteResult,
    goto,
  } from "@mateothegreat/svelte5-router";

  import Home from "$routes/Home.svelte";
  import NotFound from "$routes/NotFound.svelte";
  import Settings from "$routes/Settings.svelte";
  import MonsterList from "$routes/MonsterList.svelte";
  import MonsterBuilder from "$routes/MonsterBuilder.svelte";
  import MonsterView from "$routes/MonsterView.svelte";
  import EncounterList from "$routes/EncounterList.svelte";
  import EncounterBuilder from "$routes/EncounterBuilder.svelte";
  import EncounterView from "$routes/EncounterView.svelte";
  import SpellList from "$routes/SpellList.svelte";
  import SpellBuilder from "$routes/SpellBuilder.svelte";

  import Navbar from "$components/Navbar.svelte";
  import { Toaster } from "$components/ui/sonner";
  import { onMount } from "svelte";

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
      path: "monsters/view/(.*?)",
      component: MonsterView,
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
      path: "encounters/view/(.*?)",
      component: EncounterView,
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

  onMount(async () => {
    const route = window.location.pathname;
    goto(route);
  });
</script>

<Toaster duration={5000} position="bottom-right" />
<Navbar />
<main>
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
</main>
