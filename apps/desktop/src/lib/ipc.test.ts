// SPDX-License-Identifier: MIT
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, describe, expect, it } from "vitest";
import { createTauriIpc, type EngineSignal } from "./ipc";

afterEach(clearMocks);

describe("Tauri IPC", () => {
  it("wraps commands in the protocol envelope with increasing ids", async () => {
    const calls: [string, unknown][] = [];
    mockIPC((cmd, args) => {
      calls.push([cmd, args]);
    });
    const ipc = createTauriIpc();
    await ipc.startEngine();
    expect(await ipc.send({ type: "hello" })).toBe(1);
    expect(await ipc.send({ type: "shutdown" })).toBe(2);
    expect(calls).toEqual([
      ["engine_start", {}],
      ["engine_send", { message: { v: 0, id: 1, type: "hello" } }],
      ["engine_send", { message: { v: 0, id: 2, type: "shutdown" } }],
    ]);
  });

  it("rejects with the shell's message as an Error", async () => {
    mockIPC(() => {
      throw "The engine isn't running.";
    });
    await expect(createTauriIpc().send({ type: "hello" })).rejects.toThrow(
      "The engine isn't running.",
    );
  });

  it("delivers engine events and exits until unsubscribed", async () => {
    mockIPC(() => undefined, { shouldMockEvents: true });
    const { emit } = await import("@tauri-apps/api/event");
    const signals: EngineSignal[] = [];
    const off = await createTauriIpc().subscribe((signal) => signals.push(signal));
    await emit("engine:event", { v: 0, id: 1, type: "ready", version: "1.0.0" });
    await emit("engine:exit", "stopped");
    off();
    await emit("engine:exit", "ignored");
    expect(signals).toEqual([
      { kind: "event", message: { v: 0, id: 1, type: "ready", version: "1.0.0" } },
      { kind: "exit", reason: "stopped" },
    ]);
  });
});
