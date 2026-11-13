// SPDX-License-Identifier: MIT

/**
 * Colour math for the contrast check: OKLCH → sRGB, alpha compositing and
 * WCAG 2 relative luminance / contrast ratio. Values are gamut-clipped, as a
 * browser would display them.
 */

/** An sRGB colour with gamma-encoded channels and alpha, all in 0–1. */
export interface Rgba {
  r: number;
  g: number;
  b: number;
  a: number;
}

const OKLCH = /^oklch\(\s*([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*(?:\/\s*([\d.]+%?))?\s*\)$/;

const clamp01 = (x: number): number => Math.min(1, Math.max(0, x));

/** sRGB transfer function: linear light → gamma-encoded. */
function encode(c: number): number {
  return c <= 0.0031308 ? 12.92 * c : 1.055 * c ** (1 / 2.4) - 0.055;
}

/** Inverse sRGB transfer function: gamma-encoded → linear light. */
function decode(c: number): number {
  return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
}

/** Parses `oklch(L C H)` or `oklch(L C H / A)` (A as 0–1 or a percentage). */
export function parseOklch(value: string): Rgba {
  const m = OKLCH.exec(value.trim());
  if (m === null) {
    throw new Error(`not an oklch() colour: ${value}`);
  }
  const [, ls, cs, hs, as] = m;
  const l = Number(ls);
  const c = Number(cs);
  const h = (Number(hs) * Math.PI) / 180;
  let a = 1;
  if (as !== undefined) {
    a = as.endsWith("%") ? Number(as.slice(0, -1)) / 100 : Number(as);
  }

  // OKLCH → OKLab → LMS (cubed) → linear sRGB (Björn Ottosson's matrices).
  const labA = c * Math.cos(h);
  const labB = c * Math.sin(h);
  const l_ = (l + 0.3963377774 * labA + 0.2158037573 * labB) ** 3;
  const m_ = (l - 0.1055613458 * labA - 0.0638541728 * labB) ** 3;
  const s_ = (l - 0.0894841775 * labA - 1.291485548 * labB) ** 3;
  const r = 4.0767416621 * l_ - 3.3077115913 * m_ + 0.2309699292 * s_;
  const g = -1.2684380046 * l_ + 2.6097574011 * m_ - 0.3413193965 * s_;
  const b = -0.0041960863 * l_ - 0.7034186147 * m_ + 1.707614701 * s_;
  return { r: encode(clamp01(r)), g: encode(clamp01(g)), b: encode(clamp01(b)), a: clamp01(a) };
}

/** Paints `top` over an opaque `bottom`, blending in gamma-encoded sRGB like browsers do. */
export function composite(top: Rgba, bottom: Rgba): Rgba {
  const mix = (t: number, b: number): number => t * top.a + b * (1 - top.a);
  return { r: mix(top.r, bottom.r), g: mix(top.g, bottom.g), b: mix(top.b, bottom.b), a: 1 };
}

/** WCAG 2 relative luminance of an opaque colour. */
export function luminance({ r, g, b }: Rgba): number {
  return 0.2126 * decode(r) + 0.7152 * decode(g) + 0.0722 * decode(b);
}

/** WCAG 2 contrast ratio between two opaque colours (1–21). */
export function contrastRatio(x: Rgba, y: Rgba): number {
  const [hi, lo] = [luminance(x), luminance(y)].sort((p, q) => q - p) as [number, number];
  return (hi + 0.05) / (lo + 0.05);
}
