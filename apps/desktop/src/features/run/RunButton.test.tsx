// SPDX-License-Identifier: MIT
import { TooltipProvider } from "@barrage/ui";
import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { RunButton } from "./RunButton";

afterEach(cleanup);

describe("RunButton", () => {
  it("renders the brand Run button", () => {
    render(<RunButton />);
    const run = screen.getByRole("button", { name: "Run" });
    expect(run.className).toContain("bg-brand");
    expect(run.className).toContain("h-8");
  });

  it("shows the action and its shortcut in a tooltip on keyboard focus", async () => {
    render(
      <TooltipProvider delay={0}>
        <RunButton />
      </TooltipProvider>,
    );
    const run = screen.getByRole("button", { name: "Run" });
    act(() => {
      fireEvent.keyDown(document.body, { key: "Tab" });
      run.focus();
    });
    const kbd = await screen.findByText("⌘R");
    expect(kbd.tagName).toBe("KBD");
    expect(screen.getByText("Run plan")).toBeDefined();
  });
});
