// SPDX-License-Identifier: MIT
import { TooltipProvider } from "@barrage/ui";
import { EngineProvider } from "../features/engine";
import type { Ipc } from "../lib/ipc";
import { Editor } from "./Editor";
import { ResultsDrawer } from "./ResultsDrawer";
import { Sidebar } from "./Sidebar";
import { Toolbar } from "./Toolbar";

/**
 * Window layout from DESIGN §3.1: a 52 px toolbar across the top, the plan
 * sidebar on the left, and the editor above the (collapsed) results drawer.
 * `TooltipProvider` gives every tooltip the shared 500 ms delay;
 * `EngineProvider` connects to the engine through `ipc` (the mock in tests).
 */
export function App({ ipc }: { ipc: Ipc }) {
  return (
    <EngineProvider ipc={ipc}>
      <TooltipProvider>
        <div className="flex h-full flex-col">
          <Toolbar />
          <div className="flex min-h-0 flex-1">
            <Sidebar />
            <div className="flex min-w-0 flex-1 flex-col bg-background">
              <Editor />
              <ResultsDrawer />
            </div>
          </div>
        </div>
      </TooltipProvider>
    </EngineProvider>
  );
}
