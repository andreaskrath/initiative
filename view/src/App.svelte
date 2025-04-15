<script lang="ts">
  import Counter from "./lib/Counter.svelte";
  import "./app.css";
</script>

<main>
  <h1>Combat Tracker</h1>

  <div class="flex content-center items-center place-content-center">
    <Counter />
  </div>

  <p>
    Check out <a
      href="https://github.com/sveltejs/kit#readme"
      target="_blank"
      rel="noreferrer">SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>
</main>
