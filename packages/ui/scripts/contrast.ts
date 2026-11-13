// SPDX-License-Identifier: MIT

/**
 * WCAG AA contrast check for the text tokens in `tokens.css` (DESIGN §9).
 * Run `node scripts/contrast.ts` (it runs in `just check`); exits 1 on failure.
 */

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { composite, contrastRatio, parseOklch, type Rgba } from "./color.ts";
import { parseTokens, type Theme, type TokenMap } from "./tokens.ts";

/** A text token and a surface it's drawn on. */
export interface Pair {
  text: string;
  surface: string;
  /** Minimum ratio: 4.5 for body text (WCAG 1.4.3). */
  min: number;
}

/** One checked pair in one theme. */
export interface Result extends Pair {
  theme: Theme;
  ratio: number;
  pass: boolean;
}

const AA = 4.5;

/** Every text-on-surface combination the components use. */
export const PAIRS: readonly Pair[] = [
  ...["--background", "--sidebar", "--muted", "--popover", "--card"].flatMap((surface) => [
    { text: "--foreground", surface, min: AA },
    { text: "--muted-foreground", surface, min: AA },
  ]),
  { text: "--popover-foreground", surface: "--popover", min: AA },
  { text: "--card-foreground", surface: "--card", min: AA },
  { text: "--primary-foreground", surface: "--primary", min: AA },
  { text: "--secondary-foreground", surface: "--secondary", min: AA },
  { text: "--accent-foreground", surface: "--accent", min: AA },
  { text: "--brand-foreground", surface: "--brand", min: AA },
  // Inline validation messages are 12 px destructive text (DESIGN §7.3).
  { text: "--destructive", surface: "--background", min: AA },
];

function color(tokens: TokenMap, name: string): Rgba {
  const value = tokens.get(name);
  if (value === undefined) throw new Error(`tokens.css: ${name} is not defined`);
  return parseOklch(value);
}

/**
 * Checks every pair in both themes. Translucent surfaces (the sidebar) are
 * composited over `--background`, standing in for the window vibrancy.
 */
export function checkContrast(css: string): Result[] {
  const themes = parseTokens(css);
  return (Object.keys(themes) as Theme[]).flatMap((theme) => {
    const tokens = themes[theme];
    const base = color(tokens, "--background");
    return PAIRS.map((pair) => {
      const surface = composite(color(tokens, pair.surface), base);
      const text = composite(color(tokens, pair.text), surface);
      const ratio = contrastRatio(text, surface);
      return { ...pair, theme, ratio, pass: ratio >= pair.min };
    });
  });
}

function main(): void {
  const path = fileURLToPath(new URL("../tokens.css", import.meta.url));
  const results = checkContrast(readFileSync(path, "utf8"));
  for (const r of results) {
    const mark = r.pass ? "ok  " : "FAIL";
    const ratio = r.ratio.toFixed(2).padStart(5);
    console.log(`${mark} ${r.theme.padEnd(5)} ${ratio}:1  ${r.text} on ${r.surface}`);
  }
  const failed = results.filter((r) => !r.pass).length;
  console.log(failed === 0 ? "contrast: all pass (WCAG AA)" : `contrast: ${failed} failing`);
  process.exitCode = failed === 0 ? 0 : 1;
}

if (import.meta.main) {
  main();
}
