import type { AppRouter } from "@sfls/gateway";

import { inferRouterOutputs } from "@trpc/server";
import { createTRPCContext } from "@trpc/tanstack-react-query";

export const { TRPCProvider, useTRPC, useTRPCClient } = createTRPCContext<AppRouter>();
export type RouterOutput = inferRouterOutputs<AppRouter>;
