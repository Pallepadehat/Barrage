// SPDX-License-Identifier: MIT
import { afterEach, describe, expect, it, vi } from "vitest";
import { createMockIpc } from "../../lib/ipc-mock";
import { createEngineConnection, type EngineStatus, NO_ANSWER } from "./engine-connection";

type Connection = ReturnType<typeof createEngineConnection>;

/** Resolves with the status once it's in `state` (right away if it already is). */
function reach(connection: Connection, state: EngineStatus["state"]): Promise<EngineStatus> {
  return new Promise((resolve) => {
    const check = () => {
      const status = connection.getSnapshot();
      if (status.state !== state) return false;
      off();
      resolve(status);
      return true;
    };
    const off = connection.subscribe(check);
    check();
  });
}

afterEach(() => {
  vi.useRealTimers();
});

describe("engine connection", () => {
  it("starts the engine, says hello and becomes ready with its version", async () => {
    const ipc = createMockIpc({ version: "1.2.3" });
    const connection = createEngineConnection(ipc);
    expect(connection.getSnapshot()).toEqual({ state: "starting" });
    const close = connection.open();
    expect(await reach(connection, "ready")).toEqual({ state: "ready", version: "1.2.3" });
    expect(ipc.sent).toEqual([{ v: 0, id: 1, type: "hello" }]);
    close();
  });

  it("reports an exit with its reason, and restarts", async () => {
    const ipc = createMockIpc({ version: "1.2.3" });
    const connection = createEngineConnection(ipc);
    const close = connection.open();
    await reach(connection, "ready");

    ipc.kill("Engine gone.");
    expect(await reach(connection, "stopped")).toEqual({
      state: "stopped",
      reason: "Engine gone.",
    });

    const restarted = connection.restart();
    expect(connection.getSnapshot()).toEqual({ state: "starting" });
    await restarted;
    expect(await reach(connection, "ready")).toEqual({ state: "ready", version: "1.2.3" });
    expect(ipc.sent.map((m) => m.id)).toEqual([1, 2]);
    close();
  });

  it("reports a failed start", async () => {
    const ipc = createMockIpc({ startError: "Couldn't start the engine: not found" });
    const connection = createEngineConnection(ipc);
    const close = connection.open();
    expect(await reach(connection, "stopped")).toEqual({
      state: "stopped",
      reason: "Couldn't start the engine: not found",
    });
    close();
  });

  it("gives up when the engine never answers", async () => {
    vi.useFakeTimers();
    const connection = createEngineConnection(createMockIpc({ respond: false }), {
      helloTimeoutMs: 1000,
    });
    const close = connection.open();
    await vi.advanceTimersByTimeAsync(999);
    expect(connection.getSnapshot().state).toBe("starting");
    await vi.advanceTimersByTimeAsync(1);
    expect(connection.getSnapshot()).toEqual({ state: "stopped", reason: NO_ANSWER });
    close();
  });

  it("does nothing when closed before it's listening", async () => {
    const ipc = createMockIpc();
    const connection = createEngineConnection(ipc);
    connection.open()();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(ipc.sent).toEqual([]);
    ipc.kill();
    await new Promise((resolve) => setTimeout(resolve, 0));
    expect(connection.getSnapshot()).toEqual({ state: "starting" });
  });
});
