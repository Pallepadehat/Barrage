// SPDX-License-Identifier: MIT
import type { EngineSignal, Ipc, Unsubscribe } from "../../lib/ipc";

/** The engine as the UI sees it. */
export type EngineStatus =
  /** Spawning the sidecar and waiting for `ready`. */
  | { readonly state: "starting" }
  /** Handshake done; `version` is the engine's. */
  | { readonly state: "ready"; readonly version: string }
  /** Not running; `reason` says why and what to do, ready to show. */
  | { readonly state: "stopped"; readonly reason: string };

/** Keeps the engine's status, for `useSyncExternalStore`. */
export interface EngineConnection {
  /** The current status. Stable between changes. */
  getSnapshot(): EngineStatus;
  /** Calls `onChange` after every status change. */
  subscribe(onChange: () => void): Unsubscribe;
  /** Listens for engine signals, then starts the engine and says hello. Returns a close function. */
  open(): () => void;
  /** Starts the engine again after it stopped (or retries the handshake). */
  restart(): Promise<void>;
}

/** Options for {@link createEngineConnection}. */
export interface EngineConnectionOptions {
  /** How long to wait for `ready` before giving up. */
  readonly helloTimeoutMs?: number;
}

/** Shown when the engine doesn't answer `hello` in time. */
export const NO_ANSWER = "The engine didn't respond. Restart it to try again.";

const messageOf = (cause: unknown): string =>
  cause instanceof Error ? cause.message : String(cause);

/**
 * Creates the connection that drives the engine handshake: start the sidecar,
 * send `hello`, and turn `ready`, errors and exits into an {@link EngineStatus}.
 * Nothing here blocks: a dead or silent engine only changes the status.
 */
export function createEngineConnection(
  ipc: Ipc,
  { helloTimeoutMs = 5000 }: EngineConnectionOptions = {},
): EngineConnection {
  let status: EngineStatus = { state: "starting" };
  const watchers = new Set<() => void>();
  let helloId: number | undefined;
  let timer: ReturnType<typeof setTimeout> | undefined;

  const set = (next: EngineStatus) => {
    status = next;
    for (const watcher of watchers) watcher();
  };
  const clearTimer = () => {
    clearTimeout(timer);
    timer = undefined;
  };

  const handle = (signal: EngineSignal) => {
    if (signal.kind === "exit") {
      clearTimer();
      set({ state: "stopped", reason: signal.reason });
      return;
    }
    const { message } = signal;
    if (message.type === "ready") {
      clearTimer();
      set({ state: "ready", version: message.version });
    } else if (message.id !== undefined && message.id === helloId && status.state === "starting") {
      clearTimer();
      set({ state: "stopped", reason: message.message });
    }
  };

  const restart = async () => {
    clearTimer();
    set({ state: "starting" });
    try {
      await ipc.startEngine();
      helloId = await ipc.send({ type: "hello" });
    } catch (cause) {
      set({ state: "stopped", reason: messageOf(cause) });
      return;
    }
    // `ready` may already have arrived while `send` was resolving.
    if (status.state === "starting") {
      timer = setTimeout(() => {
        if (status.state === "starting") set({ state: "stopped", reason: NO_ANSWER });
      }, helloTimeoutMs);
    }
  };

  return {
    getSnapshot: () => status,
    subscribe(onChange) {
      watchers.add(onChange);
      return () => {
        watchers.delete(onChange);
      };
    },
    open() {
      let closed = false;
      let unsubscribe: Unsubscribe | undefined;
      ipc
        .subscribe((signal) => {
          if (!closed) handle(signal);
        })
        .then(
          (off) => {
            if (closed) {
              off();
              return;
            }
            unsubscribe = off;
            void restart();
          },
          (cause: unknown) => {
            if (!closed) set({ state: "stopped", reason: messageOf(cause) });
          },
        );
      return () => {
        closed = true;
        clearTimer();
        unsubscribe?.();
      };
    },
    restart,
  };
}
