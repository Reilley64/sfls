import type { CreateHTTPContextOptions } from "@trpc/server/adapters/standalone";

export const createContext = async (opts: CreateHTTPContextOptions) => {
  const headers = new Headers();
  if (opts.req.headers.authorization && typeof opts.req.headers.authorization === "string") {
    headers.append("Authorization", opts.req.headers.authorization);
  }
  return { headers };
};

export type Context = Awaited<ReturnType<typeof createContext>>;
