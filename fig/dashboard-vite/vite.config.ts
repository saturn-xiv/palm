import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  base: "/my/",
  server: {
    host: "0.0.0.0",
    port: 3000,
    strictPort: true,
    open: false,
    proxy: {
      "/api": "http://localhost:8080",
      "/graphql": "http://localhost:8080",
    },
  },
  plugins: [react()],
});
