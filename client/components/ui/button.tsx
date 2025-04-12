import * as React from "react";
import { ComponentRef, forwardRef } from "react";
import { Pressable, PressableProps } from "react-native";

import { type VariantProps, cva } from "class-variance-authority";

import { cn } from "~/lib/utils";

const buttonVariants = cva(
  "inline-flex flex-row items-center justify-center gap-2 rounded-md transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default: "bg-primary focus:bg-primary/90",
        destructive: "bg-destructive focus:bg-destructive/90",
        outline: "border border-input bg-background focus:bg-accent",
        secondary: "bg-secondary focus:bg-secondary/80",
        ghost: "focus:bg-accent",
        link: "underline-offset-4 focus:underline",
      },
      size: {
        default: "h-10 px-4 py-2",
        sm: "h-9 px-3",
        lg: "h-11 px-8",
        icon: "h-10 w-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  },
);

export interface ButtonProps extends PressableProps, VariantProps<typeof buttonVariants> {}

const Button = forwardRef<ComponentRef<typeof Pressable>, ButtonProps>(
  ({ className, variant, size, ...props }, ref) => {
    return (
      <Pressable className={cn(buttonVariants({ variant, size, className }))} ref={ref} role="button" {...props} />
    );
  },
);
Button.displayName = "Button";

export { Button, buttonVariants };
