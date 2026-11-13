// SPDX-License-Identifier: MIT
import { describe, expect, it } from "vitest";
import { composite, contrastRatio, luminance, parseOklch } from "./color.ts";

describe("parseOklch", () => {
  it("maps the achromatic ends to white and black", () => {
    const white = parseOklch("oklch(1 0 0)");
    for (const c of [white.r, white.g, white.b, white.a]) expect(c).toBeCloseTo(1, 6);
    const black = parseOklch("oklch(0 0 0)");
    expect([black.r, black.g, black.b]).toEqual([0, 0, 0]);
  });

  it("matches a known sRGB colour", () => {
    // oklch(0.62796 0.25768 29.2339) is #ff0000.
    const red = parseOklch("oklch(0.62796 0.25768 29.2339)");
    expect(red.r).toBeCloseTo(1, 3);
    expect(red.g).toBeCloseTo(0, 3);
    expect(red.b).toBeCloseTo(0, 3);
  });

  it("reads alpha as a fraction or a percentage", () => {
    expect(parseOklch("oklch(1 0 0 / 0.72)").a).toBe(0.72);
    expect(parseOklch("oklch(1 0 0 / 9%)").a).toBeCloseTo(0.09);
  });

  it("rejects anything that isn't oklch()", () => {
    expect(() => parseOklch("#fff")).toThrow(/not an oklch/);
  });
});

describe("contrast", () => {
  const white = parseOklch("oklch(1 0 0)");
  const black = parseOklch("oklch(0 0 0)");

  it("gives 21:1 for black on white and 1:1 for equal colours", () => {
    expect(contrastRatio(black, white)).toBeCloseTo(21, 5);
    expect(contrastRatio(white, black)).toBeCloseTo(21, 5);
    expect(contrastRatio(white, white)).toBe(1);
  });

  it("composites translucent colours in gamma space", () => {
    const half = composite({ ...white, a: 0.5 }, black);
    expect(half.r).toBeCloseTo(0.5);
    expect(luminance(half)).toBeCloseTo(0.214, 3);
  });
});
