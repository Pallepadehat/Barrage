// SPDX-License-Identifier: MIT
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { createMockIpc } from "../lib/ipc-mock";
import { App } from "./App";

afterEach(cleanup);

describe("App shell", () => {
  it("renders the toolbar as a window drag region", async () => {
    render(<App ipc={createMockIpc()} />);
    const toolbar = screen.getByRole("banner");
    expect(toolbar.hasAttribute("data-tauri-drag-region")).toBe(true);
    expect(toolbar.textContent).toContain("Untitled plan");
    expect(screen.getByRole("button", { name: "Run" }).hasAttribute("data-tauri-drag-region")).toBe(
      false,
    );
    await screen.findByText(/^Engine /);
  });

  it("renders the sidebar, editor and collapsed results drawer", async () => {
    render(<App ipc={createMockIpc()} />);
    expect(screen.getByRole("navigation", { name: "Plan" })).toBeDefined();
    expect(screen.getByRole("main")).toBeDefined();
    expect(screen.getByRole("region", { name: "Results" })).toBeDefined();
    await screen.findByText(/^Engine /);
  });
});

describe("engine status in the toolbar", () => {
  it("shows the engine version from ready, an error when the engine dies, and restarts it", async () => {
    const ipc = createMockIpc({ version: "1.2.3" });
    render(<App ipc={ipc} />);
    const status = screen.getByRole("status");
    expect(status.textContent).toBe("Starting engine…");

    expect((await screen.findByText("Engine 1.2.3")).getAttribute("role")).toBe("status");

    act(() => ipc.kill("The engine stopped unexpectedly (signal 9). Restart it to continue."));
    await screen.findByText("Engine stopped");
    expect(screen.getByRole("status").textContent).toContain("(signal 9)");
    // The rest of the UI keeps working while the engine is down.
    expect(screen.getByRole("button", { name: "Run" })).toBeDefined();

    fireEvent.click(screen.getByRole("button", { name: "Restart" }));
    await screen.findByText("Engine 1.2.3");
    expect(screen.queryByRole("button", { name: "Restart" })).toBeNull();
    expect(ipc.sent.filter((m) => m.type === "hello")).toHaveLength(2);
  });
});
