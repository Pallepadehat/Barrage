// SPDX-License-Identifier: MIT

/**
 * The one registry of app actions and their shortcuts (DESIGN §7.1). The
 * native menu, the command palette and tooltips all read from here so they
 * can't drift apart. Key bindings and the menu are wired up in M2.
 */

/** An app action with its label and shortcut. */
export interface Shortcut {
  /** Microcopy for menus, the palette and tooltips (DESIGN §10). */
  readonly label: string;
  /** The shortcut as shown to people, in macOS glyphs. */
  readonly keys: string;
}

/** Every registered action, by id. */
export const SHORTCUTS = {
  run: { label: "Run plan", keys: "⌘R" },
} as const satisfies Record<string, Shortcut>;
