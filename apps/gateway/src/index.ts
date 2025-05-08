import { TRPCError } from "@trpc/server";
import { createHTTPServer } from "@trpc/server/adapters/standalone";
import { ResultAsync } from "neverthrow";
import { z } from "zod";

import { publicProcedure, router } from "./trpc";

const SERVER_PORT = process.env.SERVER_PORT || 3000;
const BASE_URL = process.env.BASE_URL || "http://localhost:8080";

const mediaSchema = z.object({
  id: z.string(),
  createdAt: z.string(),
  updatedAt: z.string(),
  type: z.string(),
  libraryId: z.string(),
  title: z.string(),
  season: z.number().nullable(),
  episode: z.number().nullable(),
  attributes: z.any(),
  parentId: z.string().nullable(),
});

type Media = z.infer<typeof mediaSchema>;

const sessionSchema = z.object({ token: z.string() });

type Session = z.infer<typeof sessionSchema>;

type Input =
  | {
      params?: Record<string, string>;
      query?: Record<string, string | string[] | undefined>;
      headers?: Record<string, string>;
      body?: Record<string, unknown>;
    }
  | undefined;

async function callService<TOutput>(endpoint: string, opts: { input?: Input; request?: RequestInit }) {
  let url = `${BASE_URL}${endpoint}`;
  if (opts.input?.params) {
    Object.entries(opts.input.params).forEach(([key, value]) => {
      url = url.replace(`:${key}`, value);
    });
  }

  const urlObj = new URL(url);
  if (opts.input?.query) {
    Object.entries(opts.input.query).forEach(([key, value]) => {
      if (value === undefined) return;

      if (typeof value === "string") {
        urlObj.searchParams.append(key, value);
        return;
      }

      if (Array.isArray(value)) {
        value.forEach((v) => urlObj.searchParams.append(key, v));
        return;
      }
    });
  }

  if (opts?.input?.body) {
    if (!opts.input.headers) opts.input.headers = {};
    opts.input.headers["Content-Type"] = "application/json";
  }

  return ResultAsync.fromPromise(
    fetch(urlObj, {
      ...opts.request,
      headers: opts.input?.headers,
      body: opts.input?.body ? JSON.stringify(opts.input.body) : undefined,
    }).then((response) => {
      if (!response.ok) {
        throw new Error(response.statusText);
      }
      return response.json() as Promise<TOutput>;
    }),
    (err) => (err instanceof Error ? new Error(err.message) : new Error(err as string)),
  );
}

export const loggedProcedure = publicProcedure.use(async (opts) => {
  const start = Date.now();

  const result = await opts.next();

  const durationMs = Date.now() - start;
  const meta = { path: opts.path, type: opts.type, durationMs };

  result.ok ? console.log("OK request timing:", meta) : console.error("Non-OK request timing", meta);

  return result;
});

const appRouter = router({
  media: {
    get: loggedProcedure
      .input(
        z.object({
          query: z
            .object({ types: z.array(z.string()).optional(), orderBy: z.enum(["Title", "Random"]).optional() })
            .optional(),
          headers: z.object({ Authorization: z.string() }),
        }),
      )
      .output(z.array(mediaSchema))
      .query(async ({ input }) => {
        const result = await callService<Array<Media>>("/media", { input });
        return result.match(
          (data) => data,
          (error) => {
            throw new TRPCError({
              code: "INTERNAL_SERVER_ERROR",
              message: error.message,
              cause: error,
            });
          },
        );
      }),
    _media_id: {
      get: loggedProcedure
        .input(
          z.object({
            params: z.object({
              mediaId: z.string(),
            }),
            headers: z.object({ Authorization: z.string() }),
          }),
        )
        .output(mediaSchema)
        .query(async ({ input }) => {
          const result = await callService<Media>("/media/:mediaId", { input });
          return result.match(
            (data) => data,
            (error) => {
              throw new TRPCError({
                code: "INTERNAL_SERVER_ERROR",
                message: error.message,
                cause: error,
              });
            },
          );
        }),
      stream: {
        heartbeat: {
          post: loggedProcedure
            .input(
              z.object({
                params: z.object({
                  mediaId: z.string(),
                }),
                headers: z.object({ Authorization: z.string() }),
                body: z.object({
                  position: z.number(),
                }),
              }),
            )
            .output(mediaSchema)
            .mutation(async ({ input }) => {
              const result = await callService<Media>("/media/:mediaId/stream/heartbeat", {
                input,
                request: { method: "POST" },
              });
              return result.match(
                (data) => data,
                (error) => {
                  throw new TRPCError({
                    code: "INTERNAL_SERVER_ERROR",
                    message: error.message,
                    cause: error,
                  });
                },
              );
            }),
        },
      },
    },
    continue: {
      get: loggedProcedure
        .input(z.object({ headers: z.object({ Authorization: z.string() }) }))
        .output(z.array(mediaSchema))
        .query(async ({ input }) => {
          const result = await callService<Array<Media>>("/media/continue", { input });
          return result.match(
            (data) => data,
            (error) => {
              throw new TRPCError({
                code: "INTERNAL_SERVER_ERROR",
                message: error.message,
                cause: error,
              });
            },
          );
        }),
    },
  },
  sessions: {
    post: loggedProcedure
      .input(
        z.object({
          body: z.object({ email: z.string(), password: z.string() }),
        }),
      )
      .output(sessionSchema)
      .mutation(async ({ input }) => {
        const result = await callService<Session>("/sessions", { input, request: { method: "POST" } });
        return result.match(
          (data) => data,
          (error) => {
            throw new TRPCError({
              code: "INTERNAL_SERVER_ERROR",
              message: error.message,
              cause: error,
            });
          },
        );
      }),
  },
});

const server = createHTTPServer({
  router: appRouter,
});

server.listen(SERVER_PORT);

export type AppRouter = typeof appRouter;
