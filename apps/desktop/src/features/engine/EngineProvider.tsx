// SPDX-License-Identifier: MIT
import { createContext, type ReactNode, use, useEffect, useState } from "react";
import type { Ipc } from "../../lib/ipc";
import { createEngineConnection, type EngineConnection } from "./engine-connection";

const EngineContext = createContext<EngineConnection | null>(null);

/**
 * Connects to the engine through `ipc` while mounted and shares the
 * connection with {@link useEngineConnection}.
 */
export function EngineProvider({ ipc, children }: { ipc: Ipc; children: ReactNode }) {
  const [connection] = useState(() => createEngineConnection(ipc));
  useEffect(() => connection.open(), [connection]);
  return <EngineContext value={connection}>{children}</EngineContext>;
}

/** The engine connection from the nearest {@link EngineProvider}. */
export function useEngineConnection(): EngineConnection {
  const connection = use(EngineContext);
  if (connection === null) throw new Error("useEngineConnection needs an EngineProvider");
  return connection;
}
