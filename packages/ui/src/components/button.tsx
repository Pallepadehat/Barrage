// SPDX-License-Identifier: MIT
import { Button as ButtonPrimitive } from "@base-ui/react/button";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "cn";

/**
 * shadcn `Button` (base-nova) with the DESIGN §4.1 changes: sizes `sm` h-6,
 * `default` h-7, `lg` h-8 (Run); a `brand` variant for Run; `destructive` as
 * an outline (Stop, §3.3) so its text stays AA; padding on the 4 px grid;
 * 13 px text, 6 px radius, a 2 px offset focus ring, and colour changes
 * without a tween.
 */
const buttonVariants = cva(
  "group/button inline-flex shrink-0 items-center justify-center rounded-md border border-transparent bg-clip-padding text-[13px] font-medium whitespace-nowrap transition-transform duration-(--motion-fast) ease-fast outline-none select-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background active:not-aria-[haspopup]:translate-y-px disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/80",
        brand: "bg-brand text-brand-foreground hover:bg-brand/90",
        outline:
          "border-border bg-background hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50",
        secondary:
          "bg-secondary text-secondary-foreground hover:bg-secondary/80 aria-expanded:bg-secondary aria-expanded:text-secondary-foreground",
        ghost:
          "hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50",
        destructive:
          "border-destructive bg-background text-destructive hover:border-destructive/70",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        sm: "h-6 gap-1 px-2 text-xs has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 [&_svg:not([class*='size-'])]:size-3.5",
        default: "h-7 gap-1 px-3 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
        lg: "h-8 gap-2 px-3 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
        "icon-sm": "size-6 [&_svg:not([class*='size-'])]:size-3.5",
        icon: "size-7",
        "icon-lg": "size-8",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

/** Props for {@link Button}: Base UI button props plus `variant` and `size`. */
type ButtonProps = ButtonPrimitive.Props & VariantProps<typeof buttonVariants>;

/** A push button. Use `variant="brand"` only for Run (DESIGN §2.1). */
function Button({ className, variant = "default", size = "default", ...props }: ButtonProps) {
  return (
    <ButtonPrimitive
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, type ButtonProps, buttonVariants };
