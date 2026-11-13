// SPDX-License-Identifier: MIT
import { defineConfig } from "vitest/config";

/** Root Vitest config: every workspace package and app is its own test project. */
export default defineConfig({
  test: {
    projects: ["packages/*", "apps/*"],
    passWithNoTests: true,
  },
});
