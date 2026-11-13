// SPDX-License-Identifier: MIT
import { Editor } from "./Editor";
import { ResultsDrawer } from "./ResultsDrawer";
import { Sidebar } from "./Sidebar";
import { Toolbar } from "./Toolbar";

/**
 * Window layout from DESIGN §3.1: a 52 px toolbar across the top, the plan
 * sidebar on the left, and the editor above the (collapsed) results drawer.
 */
export function App() {
  return (
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
  );
}
