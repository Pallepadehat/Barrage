// SPDX-License-Identifier: MIT

/**
 * The unified title bar and toolbar. The whole bar is a window drag region;
 * the left inset leaves room for the traffic lights at x = 16.
 * `pl-24` is that clearance (16 px inset + the button cluster + a gap), not a spacing step.
 * Run, run status and the command palette button arrive in later tasks.
 */
export function Toolbar() {
  return (
    <header
      data-tauri-drag-region
      className="flex h-13 shrink-0 items-center gap-4 border-b border-border bg-background pr-4 pl-24"
    >
      <span data-tauri-drag-region className="font-semibold">
        Untitled plan
      </span>
    </header>
  );
}
