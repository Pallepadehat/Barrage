// SPDX-License-Identifier: MIT
import { Button, Kbd, Tooltip, TooltipContent, TooltipTrigger } from "@barrage/ui";
import { SHORTCUTS } from "../../lib/shortcuts";

/**
 * The toolbar's primary action (DESIGN §3.3): the app's only filled brand
 * button, with its shortcut in the tooltip. Starting a run, the Stop state
 * and the ⌘R key binding arrive with M3.
 */
export function RunButton() {
  const { label, keys } = SHORTCUTS.run;
  return (
    <Tooltip>
      <TooltipTrigger render={<Button variant="brand" size="lg" className="w-16" />}>
        Run
      </TooltipTrigger>
      <TooltipContent>
        {label}
        <Kbd>{keys}</Kbd>
      </TooltipContent>
    </Tooltip>
  );
}
