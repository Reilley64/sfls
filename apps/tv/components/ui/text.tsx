import { ComponentRef, forwardRef } from "react";
import * as React from "react";
import { Text as NativeText, TextProps } from "react-native";

import { cn } from "~/lib/utils";

const Text = forwardRef<ComponentRef<typeof NativeText>, TextProps>(({ className, ...props }, ref) => {
  return <NativeText className={cn("font-[Geist] text-foreground", className)} ref={ref} role="button" {...props} />;
});
Text.displayName = "Button";

export { Text, TextProps };
