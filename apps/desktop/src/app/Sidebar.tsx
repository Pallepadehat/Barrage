// SPDX-License-Identifier: MIT

/**
 * Caption heading for a sidebar group (DESIGN §2.2, Caption role). The nav's
 * `p-2` is the shared inset; captions and top-level tree rows each add 8 px.
 */
function Caption({ children }: { children: string }) {
  return (
    <h2 className="px-2 text-[11px] leading-[14px] font-medium tracking-wide text-muted-foreground uppercase">
      {children}
    </h2>
  );
}

/**
 * The plan sidebar: translucent fill over the window's `sidebar` vibrancy.
 * Holds the PLAN and DATA groups; the tree itself lands in M2.
 */
export function Sidebar() {
  return (
    <nav
      aria-label="Plan"
      className="flex w-62 shrink-0 flex-col overflow-y-auto gap-6 border-r border-border bg-sidebar p-2 pt-4"
    >
      <Caption>Plan</Caption>
      <Caption>Data</Caption>
    </nav>
  );
}
