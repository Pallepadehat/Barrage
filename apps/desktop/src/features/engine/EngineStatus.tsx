// SPDX-License-Identifier: MIT
import { Button, Tooltip, TooltipContent, TooltipTrigger } from "@barrage/ui";
import { useSyncExternalStore } from "react";
import { useEngineConnection } from "./EngineProvider";

/**
 * Engine state in the toolbar's status slot (DESIGN §3.3): the engine version
 * once it's ready, or "Engine stopped" with a Restart button whose tooltip
 * gives the reason. A polite live region announces the changes (§9); the
 * text is part of the window drag region, the button isn't.
 */
export function EngineStatus() {
  const connection = useEngineConnection();
  const status = useSyncExternalStore(connection.subscribe, connection.getSnapshot);
  return (
    <div data-tauri-drag-region className="flex items-center gap-2">
      <span role="status" data-tauri-drag-region className="text-muted-foreground">
        {status.state === "starting" && "Starting engine…"}
        {status.state === "ready" && `Engine ${status.version}`}
        {status.state === "stopped" && (
          <>
            <span data-tauri-drag-region className="text-destructive">
              Engine stopped
            </span>
            <span className="sr-only">. {status.reason}</span>
          </>
        )}
      </span>
      {status.state === "stopped" && (
        <Tooltip>
          <TooltipTrigger
            render={<Button variant="ghost" size="sm" onClick={() => void connection.restart()} />}
          >
            Restart
          </TooltipTrigger>
          <TooltipContent>{status.reason}</TooltipContent>
        </Tooltip>
      )}
    </div>
  );
}
