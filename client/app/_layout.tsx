import "~/global.css";

import type { AppRouter } from "~/../gateway/src";

import { Suspense, useEffect } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { KeyboardProvider } from "react-native-keyboard-controller";
import { configureReanimatedLogger } from "react-native-reanimated";
import { SafeAreaProvider } from "react-native-safe-area-context";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createTRPCClient, httpBatchLink } from "@trpc/client";
import { useFonts } from "expo-font";
import { Slot, SplashScreen } from "expo-router";

import { useAuthStore } from "~/stores/auth";

import { TRPCProvider } from "~/lib/trpc";

import { Text } from "~/components/ui/text";

import { FallbackComponent } from "~/components/FallbackComponent";

const queryClient = new QueryClient();

SplashScreen.preventAutoHideAsync();

configureReanimatedLogger({
  strict: false,
});

export default function RootLayout() {
  const auth = useAuthStore();
  const [fontLoaded, fontError] = useFonts({
    Geist: require("../assets/fonts/Geist[wght].ttf"),
  });

  useEffect(() => {
    auth.initialize();
  }, []);

  useEffect(() => {
    if (!auth.isPending && fontLoaded && !fontError) SplashScreen.hideAsync();
  }, [auth.isPending, fontLoaded, fontError]);

  const trpcClient = createTRPCClient<AppRouter>({
    links: [
      httpBatchLink({
        url: "http://192.168.86.123:10001",
        headers: () => {
          const headers = new Headers();
          if (auth.bearerToken) headers.append("Authorization", `Bearer ${auth.bearerToken}`);
          return headers;
        },
      }),
    ],
  });

  if (auth.isPending || (!fontLoaded && !fontError)) {
    return null;
  }

  return (
    <QueryClientProvider client={queryClient}>
      <TRPCProvider queryClient={queryClient} trpcClient={trpcClient}>
        <SafeAreaProvider>
          <KeyboardProvider>
            <Suspense fallback={<Text>Loading....</Text>}>
              <ErrorBoundary FallbackComponent={FallbackComponent}>
                <Slot />
              </ErrorBoundary>
            </Suspense>
          </KeyboardProvider>
        </SafeAreaProvider>
      </TRPCProvider>
    </QueryClientProvider>
  );
}
