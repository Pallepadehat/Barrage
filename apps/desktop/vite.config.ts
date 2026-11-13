// SPDX-License-Identifier: MIT
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

/**
 * Vite config for the desktop UI. Tauri loads the dev server on a fixed port
 * (see `src-tauri/tauri.conf.json`) and the built files from `dist/`.
 */
export default defineConfig({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  build: {
    // WKWebView on macOS 14, the oldest supported system.
    target: "safari17",
  },
  test: {
    environment: "jsdom",
  },
});
