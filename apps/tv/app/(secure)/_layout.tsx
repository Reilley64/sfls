import "~/global.css";

import { TextInput, View } from "react-native";

import { Stack } from "expo-router";

import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { GalleryVerticalEnd, Loader2Icon } from "lucide-react-native";
import { cssInterop } from "nativewind";
import { KeyboardAwareScrollView } from "react-native-keyboard-controller";

import { useAuthStore } from "~/stores/auth";

import { useTRPC } from "~/lib/trpc";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";

cssInterop(KeyboardAwareScrollView, {
  className: {
    target: "contentContainerStyle",
  },
});

export default function SecureLayout() {
  const auth = useAuthStore();
  const trpc = useTRPC();

  const mutation = useMutation(
    trpc.sessions.post.mutationOptions({
      onSuccess: async (data) => {
        void auth.setBearerToken(data.token);
      },
    }),
  );

  const form = useForm({
    defaultValues: {
      email: "",
      password: "",
    },
    onSubmit: async ({ value }) => {
      await mutation.mutateAsync({ body: value });
    },
  });

  if (auth.bearerToken === null) {
    return (
      <KeyboardAwareScrollView className="flex min-h-screen w-full flex-col items-center justify-center bg-background p-6 md:p-10">
        <View className="w-full max-w-sm">
          <View className="flex flex-col gap-6">
            <View className="flex flex-col items-center gap-2">
              <View className="flex h-8 w-8 flex-row items-center justify-center rounded-md">
                <GalleryVerticalEnd className="h-6 w-6" color="hsl(0 0% 98%)" />
              </View>

              <Text className="text-xl font-bold">Welcome to Selfless.</Text>
            </View>

            <View className="flex flex-col gap-6">
              <form.Field
                name="email"
                children={(field) => (
                  <View className="flex flex-col gap-2">
                    <Text className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                      Email
                    </Text>
                    <TextInput
                      id={field.name}
                      autoCapitalize="none"
                      className="flex h-11 w-full animate-none rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground transition-none file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus:border-white disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                      hasTVPreferredFocus
                      keyboardType="email-address"
                      onBlur={field.handleBlur}
                      onChangeText={(value) => field.handleChange(value)}
                      textContentType="emailAddress"
                      value={field.state.value}
                    />
                  </View>
                )}
              />

              <form.Field
                name="password"
                children={(field) => (
                  <View className="flex flex-col gap-2">
                    <Text className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                      Password
                    </Text>
                    <TextInput
                      id={field.name}
                      autoCapitalize="none"
                      className="flex h-11 w-full animate-none rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground transition-none file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus:border-white disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                      hasTVPreferredFocus
                      onBlur={field.handleBlur}
                      onChangeText={(value) => field.handleChange(value)}
                      textContentType="password"
                      value={field.state.value}
                    />
                  </View>
                )}
              />

              <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
                children={([canSubmit, isSubmitting]) => (
                  <Button disabled={!canSubmit} onPress={form.handleSubmit} size="lg">
                    {isSubmitting && <Loader2Icon className="h-4 w-4 animate-spin" color="hsl(0 0% 98%)" />}
                    <Text className="font-medium">Login</Text>
                  </Button>
                )}
              />

              {mutation.isError && (
                <View>
                  <Text className="text-destructive">{JSON.stringify(mutation.error)}</Text>
                </View>
              )}
            </View>
          </View>
        </View>
      </KeyboardAwareScrollView>
    );
  }

  return (
    <Stack
      screenOptions={{
        headerShown: false,
        animation: "none",
      }}
    />
  );
}
