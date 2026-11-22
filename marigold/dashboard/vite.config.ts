import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  base: "/my/",
  server: {
    host: "0.0.0.0",
    port: 4000,
    strictPort: true,
    proxy: {
      "/graphql": "http://localhost:3000",
      "/api": "http://localhost:3000",
    },
  },
  build: {
    chunkSizeWarningLimit: 1000,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("node_modules")) {
            return "vendor";
          }
        },
      },
    },
  },
});
