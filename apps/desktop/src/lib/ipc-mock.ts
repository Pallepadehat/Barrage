// SPDX-License-Identifier: MIT
import type { CommandMessage, EventMessage } from "@barrage/types";
import type { EngineSignal, Ipc } from "./ipc";
import { PROTOCOL_VERSION, requestIds } from "./protocol";

/** Options for {@link createMockIpc}. */
export interface MockIpcOptions {
  /** Version the mock engine reports in `ready`. */
  readonly version?: string;
  /** When false, the mock never answers `hello`, as if the engine hung. */
  readonly respond?: boolean;
  /** When set, `startEngine` rejects with this message. */
  readonly startError?: string;
}

/** An in-memory engine for tests and the browser preview. */
export interface MockIpc extends Ipc {
  /** Every command sent while the engine was running, in order. */
  readonly sent: readonly CommandMessage[];
  /** Simulates the engine process dying, as if it were killed. */
  kill(reason?: string): void;
}

/** Reason the mock reports when killed, worded like the Rust shell's. */
const KILLED = "The engine stopped unexpectedly (signal 9). Restart it to continue.";

/**
 * Creates a mock engine that behaves like the sidecar: it must be started,
 * answers `hello` with `ready`, exits on `shutdown`, and reports an exit when
 * killed. Signals are delivered asynchronously, as they are in the app.
 */
export function createMockIpc(options: MockIpcOptions = {}): MockIpc {
  const { version = "0.0.0-preview", respond = true, startError } = options;
  const nextId = requestIds();
  const listeners = new Set<(signal: EngineSignal) => void>();
  const sent: CommandMessage[] = [];
  let running = false;

  const emit = (signal: EngineSignal) => {
    queueMicrotask(() => {
      for (const listener of listeners) listener(signal);
    });
  };
  const reply = (message: EventMessage) => emit({ kind: "event", message });

  return {
    sent,
    async subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    async startEngine() {
      if (startError !== undefined) throw new Error(startError);
      running = true;
    },
    async send(command) {
      if (!running) throw new Error("The engine isn't running.");
      const id = nextId();
      sent.push({ v: PROTOCOL_VERSION, id, ...command });
      if (command.type === "shutdown") {
        running = false;
      } else if (respond) {
        reply({ v: PROTOCOL_VERSION, id, type: "ready", version });
      }
      return id;
    },
    kill(reason = KILLED) {
      if (!running) return;
      running = false;
      emit({ kind: "exit", reason });
    },
  };
}
