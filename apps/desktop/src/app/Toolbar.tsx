// SPDX-License-Identifier: MIT
import { EngineStatus } from "../features/engine";
import { RunButton } from "../features/run";

/**
 * The unified title bar and toolbar. The bar and its non-interactive parts are
 * a window drag region; Tauri only drags from elements that carry the
 * attribute, so controls stay clickable. The left inset leaves room for the
 * traffic lights at x = 16: `pl-24` is that clearance (16 px inset + the button
 * cluster + a gap), not a spacing step.
 * The status slot after Run shows the engine until runs land in M3; the
 * command palette button arrives in a later task.
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
      <div data-tauri-drag-region className="h-full flex-1" />
      <RunButton />
      <EngineStatus />
    </header>
  );
}
