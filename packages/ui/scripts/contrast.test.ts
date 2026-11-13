// SPDX-License-Identifier: MIT
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { checkContrast, isFailure, PAIRS } from "./contrast.ts";

const tokensCss = readFileSync(fileURLToPath(new URL("../tokens.css", import.meta.url)), "utf8");

describe("checkContrast", () => {
  it("passes WCAG AA for every gating pair in both themes", () => {
    const results = checkContrast(tokensCss);
    expect(results).toHaveLength(PAIRS.length * 2);
    expect(results.filter(isFailure)).toEqual([]);
  });

  it("composites tinted surfaces over the background", () => {
    const tinted = checkContrast(tokensCss).find(
      (r) => r.theme === "light" && r.text === "--foreground" && r.tint === 0.12,
    );
    const plain = checkContrast(tokensCss).find(
      (r) => r.theme === "light" && r.text === "--foreground" && r.surface === "--background",
    );
    expect(tinted?.ratio).toBeLessThan(plain?.ratio ?? 0);
    expect(tinted?.pass).toBe(true);
  });

  it("reports pending pairs without failing on them", () => {
    const pending = checkContrast(tokensCss).filter((r) => r.pending !== undefined && !r.pass);
    expect(pending.length).toBeGreaterThan(0);
    expect(pending.some(isFailure)).toBe(false);
  });

  it("flags a text token that is too light", () => {
    const css = tokensCss.replace(
      "--muted-foreground: oklch(0.52 0 0);",
      "--muted-foreground: oklch(0.8 0 0);",
    );
    const failing = checkContrast(css).filter(isFailure);
    expect(failing.length).toBeGreaterThan(0);
    expect(failing.every((r) => r.theme === "light" && r.text === "--muted-foreground")).toBe(true);
  });
});
