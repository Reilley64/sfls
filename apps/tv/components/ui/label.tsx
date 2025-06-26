import * as React from "react";

import { View } from "react-native";

import * as LabelPrimitive from "@rn-primitives/label";

import { cn } from "~/lib/utils";

const Label = React.forwardRef<LabelPrimitive.TextRef, LabelPrimitive.TextProps>(
  ({ className, ...props }, ref) => (
    <LabelPrimitive.Root
      asChild
      className="web:cursor-default"
    >
      <View>
        <LabelPrimitive.Text
          ref={ref}
          className={cn(
            "native:text-base text-sm font-medium leading-none text-foreground web:peer-disabled:cursor-not-allowed web:peer-disabled:opacity-70",
            className,
          )}
          {...props}
        />
      </View>
    </LabelPrimitive.Root>
  ),
);
Label.displayName = LabelPrimitive.Root.displayName;

export { Label };
