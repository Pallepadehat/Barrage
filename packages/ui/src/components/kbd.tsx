// SPDX-License-Identifier: MIT

import { cn } from "cn";
import type { ComponentProps } from "react";

/** A keyboard shortcut hint (DESIGN §4.2): mono 11 px on a `--muted` fill, 4 px radius. */
function Kbd({ className, ...props }: ComponentProps<"kbd">) {
  return (
    <kbd
      data-slot="kbd"
      className={cn(
        "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm bg-muted px-1 font-mono text-[11px] leading-[14px] font-medium text-muted-foreground select-none [&_svg:not([class*='size-'])]:size-3",
        className,
      )}
      {...props}
    />
  );
}

/** Groups several {@link Kbd}s, e.g. a chord. */
function KbdGroup({ className, ...props }: ComponentProps<"kbd">) {
  return (
    <kbd
      data-slot="kbd-group"
      className={cn("inline-flex items-center gap-1", className)}
      {...props}
    />
  );
}

export { Kbd, KbdGroup };
