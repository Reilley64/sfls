import "~/global.css";

import type { AppRouter } from "@sfls/gateway";

import { Suspense, useEffect } from "react";
import { SafeAreaView, View } from "react-native";

import { fetch } from "expo/fetch";
import { useFonts } from "expo-font";
import { ErrorBoundaryProps, Slot, SplashScreen } from "expo-router";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createTRPCClient, httpBatchLink } from "@trpc/client";
import { KeyboardProvider } from "react-native-keyboard-controller";
import { configureReanimatedLogger } from "react-native-reanimated";
import { SafeAreaProvider } from "react-native-safe-area-context";

import { useAuthStore } from "~/stores/auth";

import { TRPCProvider } from "~/lib/trpc";

import { Text, TextClassContext } from "~/components/ui/text";

const queryClient = new QueryClient();

SplashScreen.preventAutoHideAsync();

configureReanimatedLogger({
  strict: false,
});

export function ErrorBoundary({ error, retry }: ErrorBoundaryProps) {
  const auth = useAuthStore();

  useEffect(() => {
    async function effect() {
      if (error.message === "Unauthorized") {
        await auth.deleteBearerToken();
        await retry();
      }
    }

    void effect();
  }, [error.message, retry]);

  return (
    <View>
      <Text>{JSON.stringify(error)}</Text>
    </View>
  );
}

export default function RootLayout() {
  const auth = useAuthStore();
  const [fontLoaded, fontError] = useFonts({
    Geist: require("../assets/fonts/Geist[wght].ttf"),
  });

  useEffect(() => {
    void auth.initialize();
  }, []);

  useEffect(() => {
    if (!auth.isPending && fontLoaded && !fontError) SplashScreen.hideAsync();
  }, [auth.isPending, fontLoaded, fontError]);

  const trpcClient = createTRPCClient<AppRouter>({
    links: [
      httpBatchLink({
        url: "http://192.168.86.215:10001",
        fetch: fetch as any,
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
            <TextClassContext.Provider value="text-foreground">
              <Suspense fallback={<Text>Loading....</Text>}>
                <Slot />
              </Suspense>
            </TextClassContext.Provider>
          </KeyboardProvider>
        </SafeAreaProvider>
      </TRPCProvider>
    </QueryClientProvider>
  );
}
