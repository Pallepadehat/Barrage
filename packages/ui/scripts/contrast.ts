// SPDX-License-Identifier: MIT

/**
 * WCAG AA contrast check for the text tokens in `tokens.css` (DESIGN §9).
 * Run `node scripts/contrast.ts` (it runs in `just check`); exits 1 when a
 * gating pair fails. Pending pairs are printed but don't fail the check.
 */

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { composite, contrastRatio, parseOklch, type Rgba } from "./color.ts";
import { parseTokens, type Theme, type TokenMap } from "./tokens.ts";

/** A text token and a surface it's drawn on. */
export interface Pair {
  text: string;
  surface: string;
  /** Opacity of the surface over `--background`, for tinted fills like `bg-brand/12`. */
  tint?: number;
  /** Minimum ratio: 4.5 for body text (WCAG 1.4.3). */
  min: number;
  /**
   * Set for pairs that no shipped component uses yet and that are known to
   * fail: reported as `todo`, never failing the check. Name the task that
   * must fix it.
   */
  pending?: string;
}

/** One checked pair in one theme. */
export interface Result extends Pair {
  theme: Theme;
  ratio: number;
  pass: boolean;
}

const AA = 4.5;

/** Every text-on-surface combination the components use, or will use. */
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
  // Validation messages (§7.3) and the destructive outline button (§3.3).
  { text: "--destructive", surface: "--background", min: AA },
  // Selected tree row: foreground on a 12 % brand fill (§3.2).
  { text: "--foreground", surface: "--brand", tint: 0.12, min: AA },
  // Status colours as text: MethodBadge on the background, StatusBadge on a
  // 12 % tint (§4.2). Light mode fails today; see ADR 0003.
  ...["--success", "--brand", "--warning", "--destructive"].map((status) => ({
    text: status,
    surface: "--background",
    min: AA,
    pending: "M2 MethodBadge",
  })),
  ...["--success", "--warning", "--destructive"].map((status) => ({
    text: status,
    surface: status,
    tint: 0.12,
    min: AA,
    pending: "M3 StatusBadge",
  })),
];

function color(tokens: TokenMap, name: string): Rgba {
  const value = tokens.get(name);
  if (value === undefined) throw new Error(`tokens.css: ${name} is not defined`);
  return parseOklch(value);
}

/**
 * Checks every pair in both themes. Translucent and tinted surfaces are
 * composited over `--background`, which stands in for the window vibrancy
 * under the sidebar.
 */
export function checkContrast(css: string): Result[] {
  const themes = parseTokens(css);
  return (Object.keys(themes) as Theme[]).flatMap((theme) => {
    const tokens = themes[theme];
    const base = color(tokens, "--background");
    return PAIRS.map((pair) => {
      const fill = color(tokens, pair.surface);
      const surface = composite({ ...fill, a: fill.a * (pair.tint ?? 1) }, base);
      const text = composite(color(tokens, pair.text), surface);
      const ratio = contrastRatio(text, surface);
      return { ...pair, theme, ratio, pass: ratio >= pair.min };
    });
  });
}

/** True when a result should fail the check. */
export const isFailure = (r: Result): boolean => !r.pass && r.pending === undefined;

function main(): void {
  const path = fileURLToPath(new URL("../tokens.css", import.meta.url));
  const results = checkContrast(readFileSync(path, "utf8"));
  for (const r of results) {
    const mark = r.pass ? "ok  " : r.pending === undefined ? "FAIL" : "todo";
    const ratio = r.ratio.toFixed(2).padStart(5);
    const surface = r.tint === undefined ? r.surface : `${r.surface} @ ${r.tint * 100} %`;
    const note = !r.pass && r.pending !== undefined ? `  (before ${r.pending})` : "";
    console.log(`${mark} ${r.theme.padEnd(5)} ${ratio}:1  ${r.text} on ${surface}${note}`);
  }
  const failed = results.filter(isFailure).length;
  const todo = results.filter((r) => !r.pass && r.pending !== undefined).length;
  const summary = failed === 0 ? "all gating pairs pass (WCAG AA)" : `${failed} failing`;
  console.log(`contrast: ${summary}${todo > 0 ? `; ${todo} pending` : ""}`);
  process.exitCode = failed === 0 ? 0 : 1;
}

if (import.meta.main) {
  main();
}
