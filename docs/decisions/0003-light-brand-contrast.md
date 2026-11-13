# 0003 — Light-theme brand lightness for AA contrast

- **Status:** Accepted
- **Date:** 2026-11-13

## Context

DESIGN §2.1 set the light-theme `--brand` to `oklch(0.60 0.19 256)`, and §9 requires WCAG AA text contrast in both themes. The Run button puts its 13 px label on a `--brand` fill (§3.3), so the label needs 4.5:1. The contrast script added in M0-09 measures white on `oklch(0.60 0.19 256)` at **4.02:1**, a failure. The two DESIGN rules can't both hold.

## Decision

- Light `--brand` becomes `oklch(0.57 0.19 256)`: the smallest step in lightness that gets white text to AA (4.55:1). Chroma and hue are unchanged, so it reads as the same blue.
- `--brand-foreground` (text on a brand fill) is white in light. In dark it is the `--background` grey (6.63:1): the dark brand is lighter, so white on it would fail AA (2.9:1), and darkening it would lose contrast against the dark window. The dark `--brand` is unchanged.
- DESIGN §2.1 is updated to the new value and points here. `packages/ui/scripts/contrast.ts` runs in `just check`, so a future token edit can't regress this silently.

## Consequences

- Selected rows, focus rings and chart series in light mode are a touch darker; at 12 % fill (selection) the difference is not visible.
- Any new text-on-colour pairing should be added to the script's pair list.
- **Open:** in light mode, `--success` (3.28:1) and `--warning` (2.27:1) fail AA as text on the background, and `--success`, `--warning` and `--destructive` fail on their own 12 % tints. DESIGN §4.2 uses them that way in `MethodBadge` and `StatusBadge`. No component does yet, so the script lists these pairs as *pending* (reported, not gating). They need a decision, probably separate darker text tones for light mode in a new ADR, before those badges land in M2/M3.

## Alternatives considered

- **Dark text on the light brand** (4.80:1 with `--foreground`): keeps the old value, but dark text on a mid-blue button reads as disabled next to native macOS accent buttons; rejected. (The dark theme does the reverse because its brand is a light blue.)
- **Exempt the Run label as "large text"** (3:1): 13 px medium is not large text under WCAG; rejected.
