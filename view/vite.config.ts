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
      $components: path.resolve("./src/lib/components"),
      $types: path.resolve("./src/lib/types/"),
      $utils: path.resolve("./src/lib/utils/"),
      $routes: path.resolve("./src/lib/routes/"),
      $services: path.resolve("./src/lib/services/"),
    },
  },
});
