import { useEffect } from "react";
import { SafeAreaView } from "react-native";

import { FallbackProps } from "react-error-boundary";

import { useAuthStore } from "~/stores/auth";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";

export function FallbackComponent({ error, resetErrorBoundary }: FallbackProps) {
  const auth = useAuthStore();

  useEffect(() => {
    async function effect() {
      console.log(error.message);

      if (error.message === "Unauthorized") {
        await auth.deleteBearerToken();
        resetErrorBoundary();
      }
    }

    void effect();
  }, []);

  return (
    <SafeAreaView className="max-w-screen min-h-screen w-full bg-background p-6">
      <Text>{error.message}</Text>
      <Button onPress={resetErrorBoundary}>
        <Text>Try again</Text>
      </Button>
      <Button onPress={() => auth.deleteBearerToken()}>
        <Text>Logout</Text>
      </Button>
    </SafeAreaView>
  );
}
