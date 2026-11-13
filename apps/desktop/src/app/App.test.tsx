// SPDX-License-Identifier: MIT
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { App } from "./App";

afterEach(cleanup);

describe("App shell", () => {
  it("renders the toolbar as a window drag region", () => {
    render(<App />);
    const toolbar = screen.getByRole("banner");
    expect(toolbar.hasAttribute("data-tauri-drag-region")).toBe(true);
    expect(toolbar.textContent).toContain("Untitled plan");
  });

  it("renders the sidebar, editor and collapsed results drawer", () => {
    render(<App />);
    expect(screen.getByRole("navigation", { name: "Plan" })).toBeDefined();
    expect(screen.getByRole("main")).toBeDefined();
    expect(screen.getByRole("region", { name: "Results" })).toBeDefined();
  });
});
