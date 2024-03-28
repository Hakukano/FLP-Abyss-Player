import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],

  clearScreen: false,
  server: {
    port: 8080,
    strictPort: true,
    proxy: {
      "/api": "http://localhost:44444",
      "/stream": "http://localhost:44444",
    },
  },
}));
