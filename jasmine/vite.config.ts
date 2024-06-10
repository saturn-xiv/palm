import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  base: "/my/",
  server: {
    host: true,
    proxy: {
      "/graphql": "http://localhost:4000",
    },
  },
});
