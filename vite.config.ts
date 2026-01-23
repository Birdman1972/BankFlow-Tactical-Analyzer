import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  plugins: [svelte()],

  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },

  // Build configuration for WASM
  build: {
    target: "esnext",
    rollupOptions: {
      output: {
        // Ensure WASM files are properly handled
        assetFileNames: (assetInfo) => {
          if (assetInfo.name?.endsWith(".wasm")) {
            return "assets/wasm/[name][extname]";
          }
          return "assets/[name]-[hash][extname]";
        },
      },
    },
  },

  // Optimize dependencies
  optimizeDeps: {
    exclude: ["$lib/wasm/bankflow-core-wasm/bankflow_core.js"],
  },

  // Ensure WASM files are served correctly
  assetsInclude: ["**/*.wasm"],
});
