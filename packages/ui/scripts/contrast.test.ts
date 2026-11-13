// SPDX-License-Identifier: MIT
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { checkContrast, PAIRS } from "./contrast.ts";

const tokensCss = readFileSync(fileURLToPath(new URL("../tokens.css", import.meta.url)), "utf8");

describe("checkContrast", () => {
  it("passes WCAG AA for every pair in both themes", () => {
    const results = checkContrast(tokensCss);
    expect(results).toHaveLength(PAIRS.length * 2);
    expect(results.filter((r) => !r.pass)).toEqual([]);
  });

  it("flags a text token that is too light", () => {
    const css = tokensCss.replace(
      "--muted-foreground: oklch(0.52 0 0);",
      "--muted-foreground: oklch(0.8 0 0);",
    );
    const failing = checkContrast(css).filter((r) => !r.pass);
    expect(failing.length).toBeGreaterThan(0);
    expect(failing.every((r) => r.theme === "light" && r.text === "--muted-foreground")).toBe(true);
  });
});
