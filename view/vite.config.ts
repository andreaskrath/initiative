import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// https://vite.dev/config/
export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
      $components: path.resolve("./src/lib/shared/components/"),
      $utils: path.resolve("./src/lib/shared/utils/"),
      $routes: path.resolve("./src/lib/routes/"),
      $spell: path.resolve("./src/lib/spell/"),
      $encounter: path.resolve("./src/lib/encounter/"),
      $monster: path.resolve("./src/lib/monster/"),
      $shared: path.resolve("./src/lib/shared/"),
      $types: path.resolve("./src/lib/types/"),
    },
  },
});
