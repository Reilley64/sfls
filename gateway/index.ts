import { TRPCError } from "@trpc/server";
import { createHTTPServer } from "@trpc/server/adapters/standalone";
import { ResultAsync } from "neverthrow";
import { z } from "zod";

import { createContext } from "./context";
import { publicProcedure, router } from "./trpc";

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

async function fetchApi<T>(url: string | URL | Request, options?: RequestInit) {
  return ResultAsync.fromPromise(
    fetch(url, options).then((response) => {
      if (!response.ok) {
        throw new Error(response.statusText);
      }
      return response.json() as Promise<T>;
    }),
    (err) => (err instanceof Error ? new Error(err.message) : new Error(err as string)),
  );
}

const appRouter = router({
  media: {
    get: publicProcedure
      .input(z.object({ query: z.object({ types: z.array(z.string()).optional() }).optional() }).optional())
      .output(z.array(mediaSchema))
      .query(async ({ ctx, input }) => {
        const url = new URL("http://localhost:8080/media");
        if (input && input.query) {
          Object.entries(input.query).forEach(([key, value]) => {
            if (typeof value === "string") {
              url.searchParams.append(key, value);
              return;
            }

            if (Array.isArray(value)) {
              value.forEach((v) => url.searchParams.append(key, v));
              return;
            }
          });
        }
        const result = await fetchApi<Array<Media>>(url, { headers: ctx.headers });
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
      get: publicProcedure
        .input(
          z.object({
            params: z.object({
              mediaId: z.string(),
            }),
          }),
        )
        .output(mediaSchema)
        .query(async ({ ctx, input }) => {
          const result = await fetchApi<Media>(`http://localhost:8080/media/${input.params.mediaId}`, {
            headers: ctx.headers,
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
    continue: {
      get: publicProcedure.output(z.array(mediaSchema)).query(async ({ ctx }) => {
        const result = await fetchApi<Array<Media>>("http://localhost:8080/media/continue", { headers: ctx.headers });
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
    post: publicProcedure
      .input(z.object({ body: z.object({ email: z.string(), password: z.string() }) }))
      .output(z.object({ token: z.string() }))
      .mutation(async ({ input }) => {
        const result = await fetchApi<{ token: string }>("http://localhost:8080/sessions", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(input.body),
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
});

const server = createHTTPServer({
  router: appRouter,
  createContext,
});

server.listen(3000);

export type AppRouter = typeof appRouter;
