// SPDX-License-Identifier: MIT
import { Button, Tooltip, TooltipContent, TooltipTrigger } from "@barrage/ui";
import { useRef, useSyncExternalStore } from "react";
import { useEngineConnection } from "./EngineProvider";

/**
 * Engine state in the toolbar's status slot (DESIGN §3.3): the engine version
 * once it's ready, or "Engine stopped" with a Restart button whose tooltip
 * gives the reason. Only the error is announced (§9, live regions announce
 * errors). The text is part of the window drag region; the button isn't.
 */
export function EngineStatus() {
  const connection = useEngineConnection();
  const status = useSyncExternalStore(connection.subscribe, connection.getSnapshot);
  const label = useRef<HTMLSpanElement>(null);

  const restart = () => {
    // The button unmounts while restarting; keep keyboard focus nearby.
    label.current?.focus();
    void connection.restart();
  };

  return (
    <div data-tauri-drag-region className="flex items-center gap-2">
      <span
        ref={label}
        tabIndex={-1}
        data-tauri-drag-region
        className={
          status.state === "stopped"
            ? "text-destructive outline-none"
            : "text-muted-foreground outline-none"
        }
      >
        {status.state === "starting" && "Starting engine…"}
        {status.state === "ready" && `Engine ${status.version}`}
        {status.state === "stopped" && "Engine stopped"}
      </span>
      <span role="status" className="sr-only">
        {status.state === "stopped" ? `Engine stopped. ${status.reason}` : ""}
      </span>
      {status.state === "stopped" && (
        <Tooltip>
          <TooltipTrigger
            render={
              <Button variant="ghost" size="sm" aria-label="Restart engine" onClick={restart} />
            }
          >
            Restart
          </TooltipTrigger>
          <TooltipContent>{status.reason}</TooltipContent>
        </Tooltip>
      )}
    </div>
  );
}
