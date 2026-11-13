// SPDX-License-Identifier: MIT

/**
 * @barrage/ui — shadcn/ui components (Base UI flavour) and the design tokens
 * shared by the desktop app. Holds no app logic. Apps import `styles.css`
 * once, after Tailwind, and the components from here.
 */
export { Button, type ButtonProps, buttonVariants } from "./components/button";
export { Kbd, KbdGroup } from "./components/kbd";
export {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "./components/tooltip";
