// SPDX-License-Identifier: MIT

/** The editor pane. Shows an empty state until the plan editors land in M2. */
export function Editor() {
  return (
    <main className="min-h-0 flex-1 overflow-auto p-6">
      <div className="max-w-[880px]">
        <p className="text-muted-foreground">Select a step in the sidebar to edit it.</p>
      </div>
    </main>
  );
}
