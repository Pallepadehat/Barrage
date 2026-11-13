// SPDX-License-Identifier: MIT

/**
 * The results drawer in its collapsed state: just the header strip.
 * Opening, resizing and the result tabs land in M3.
 */
export function ResultsDrawer() {
  return (
    <section
      aria-label="Results"
      className="flex h-8 shrink-0 items-center border-t border-border px-4 text-muted-foreground"
    >
      Results
    </section>
  );
}
