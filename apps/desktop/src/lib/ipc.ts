// SPDX-License-Identifier: MIT
import type { Command, CommandMessage, EventMessage } from "@barrage/types";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createMockIpc } from "./ipc-mock";
import { PROTOCOL_VERSION, requestIds } from "./protocol";

/**
 * The app's only door to Tauri (DESIGN §11). Components and features talk to
 * the engine through an {@link Ipc}; in the app it's backed by the Rust shell,
 * which owns the sidecar process, and in tests and the browser preview by the
 * in-memory mock from `ipc-mock.ts`.
 */

/** Something the engine connection reports. */
export type EngineSignal =
  /** One protocol event from the engine's stdout. */
  | { readonly kind: "event"; readonly message: EventMessage }
  /** The engine process stopped; `reason` is ready to show in the UI. */
  | { readonly kind: "exit"; readonly reason: string };

/** Stops a subscription. */
export type Unsubscribe = () => void;

/** Typed access to the engine sidecar. */
export interface Ipc {
  /** Delivers engine signals to `listener`. Resolves once it's listening. */
  subscribe(listener: (signal: EngineSignal) => void): Promise<Unsubscribe>;
  /** Starts the engine unless it's running. Rejects with a readable message if it can't. */
  startEngine(): Promise<void>;
  /** Sends `command` to the engine and resolves with the request id its answers carry. */
  send(command: Command): Promise<number>;
}

/** Tauri event names and commands; they match `src-tauri/src/engine.rs`. */
const EVENT = "engine:event";
const EXIT = "engine:exit";

/** Turns a rejected `invoke` (the Rust side rejects with a string) into an `Error`. */
function toError(cause: unknown): Error {
  return cause instanceof Error ? cause : new Error(String(cause));
}

/** The engine as seen through the Tauri shell. */
export function createTauriIpc(): Ipc {
  const nextId = requestIds();
  return {
    async subscribe(listener) {
      const offEvent = await listen<EventMessage>(EVENT, ({ payload }) =>
        listener({ kind: "event", message: payload }),
      );
      try {
        const offExit = await listen<string>(EXIT, ({ payload }) =>
          listener({ kind: "exit", reason: payload }),
        );
        return () => {
          offEvent();
          offExit();
        };
      } catch (cause) {
        offEvent();
        throw toError(cause);
      }
    },
    async startEngine() {
      await invoke("engine_start").catch((cause: unknown) => {
        throw toError(cause);
      });
    },
    async send(command) {
      const id = nextId();
      const message: CommandMessage = { v: PROTOCOL_VERSION, id, ...command };
      await invoke("engine_send", { message }).catch((cause: unknown) => {
        throw toError(cause);
      });
      return id;
    },
  };
}

/** The Tauri-backed IPC inside the app; the mock in a plain browser (the preview). */
export function defaultIpc(): Ipc {
  return isTauri() ? createTauriIpc() : createMockIpc();
}
