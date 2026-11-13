# 0003 — Light-theme brand lightness for AA contrast

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

DESIGN §2.1 set the light-theme `--brand` to `oklch(0.60 0.19 256)`, and §9 requires WCAG AA text contrast in both themes. The Run button puts its 13 px label on a `--brand` fill (§3.3), so the label needs 4.5:1. The contrast script added in M0-09 measures white on `oklch(0.60 0.19 256)` at **4.02:1**, a failure. The two DESIGN rules can't both hold.

## Decision

- Light `--brand` becomes `oklch(0.57 0.19 256)`: the smallest step in lightness that gets white text to AA (4.55:1). Chroma and hue are unchanged, so it reads as the same blue.
- `--brand-foreground` (text on a brand fill) is white in light and `--background` grey in dark (6.63:1 on the dark brand). The dark `--brand` is unchanged.
- DESIGN §2.1 is updated to the new value and points here. `packages/ui/scripts/contrast.ts` runs in `just check`, so a future token edit can't regress this silently.

## Consequences

- Selected rows, focus rings and chart series in light mode are a touch darker; at 12 % fill (selection) the difference is not visible.
- Any new text-on-colour pairing should be added to the script's pair list.

## Alternatives considered

- **Dark text on the light brand** (4.80:1 with `--foreground`): keeps the old value but no Mac app puts dark text on an accent-blue button; rejected.
- **Exempt the Run label as "large text"** (3:1): 13 px medium is not large text under WCAG; rejected.
