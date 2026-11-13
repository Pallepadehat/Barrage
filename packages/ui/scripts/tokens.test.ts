// SPDX-License-Identifier: MIT
import { describe, expect, it } from "vitest";
import { parseTokens } from "./tokens.ts";

const CSS = `
/* :root { --ignored: 1; } */
:root {
  --a: oklch(1 0 0);
  --b: var(--a);
  --c: oklch(0.5 0 0);
}
@media (prefers-color-scheme: dark) {
  :root {
    --a: oklch(0 0 0);
  }
}
@theme inline { --color-a: var(--a); }
`;

describe("parseTokens", () => {
  it("reads light tokens and resolves var() references", () => {
    const { light } = parseTokens(CSS);
    expect(light.get("--b")).toBe("oklch(1 0 0)");
    expect(light.has("--ignored")).toBe(false);
    expect(light.has("--color-a")).toBe(false);
  });

  it("layers dark overrides on light and re-resolves references", () => {
    const { dark } = parseTokens(CSS);
    expect(dark.get("--a")).toBe("oklch(0 0 0)");
    expect(dark.get("--b")).toBe("oklch(0 0 0)");
    expect(dark.get("--c")).toBe("oklch(0.5 0 0)");
  });

  it("fails on undefined or circular references", () => {
    expect(() => parseTokens(CSS.replace("var(--a)", "var(--nope)"))).toThrow(/--nope/);
    expect(() => parseTokens(CSS.replace("--a: oklch(1 0 0)", "--a: var(--b)"))).toThrow(/itself/);
  });

  it("fails when the file has no dark override", () => {
    expect(() => parseTokens(":root { --a: oklch(1 0 0); }")).toThrow(/dark-scheme/);
  });
});
