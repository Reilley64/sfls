import "~/global.css";

import { View } from "react-native";

import { Stack } from "expo-router";

import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { GalleryVerticalEnd, Loader2Icon } from "lucide-react-native";
import { cssInterop } from "nativewind";
import { KeyboardAwareScrollView } from "react-native-keyboard-controller";

import { useAuthStore } from "~/stores/auth";

import { useTRPC } from "~/lib/trpc";

import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Label } from "~/components/ui/label";
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
                    <Label>Email</Label>
                    <Input
                      id={field.name}
                      autoCapitalize="none"
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
                    <Label>Password</Label>
                    <Input
                      id={field.name}
                      autoComplete="current-password"
                      autoCapitalize="none"
                      hasTVPreferredFocus
                      onBlur={field.handleBlur}
                      onChangeText={(value) => field.handleChange(value)}
                      secureTextEntry
                      textContentType="password"
                      value={field.state.value}
                    />
                  </View>
                )}
              />

              <form.Subscribe
                selector={(state) => [state.canSubmit, state.isSubmitting]}
                children={([canSubmit, isSubmitting]) => (
                  <Button disabled={!canSubmit} onPress={form.handleSubmit}>
                    <View className="flex flex-row items-center gap-2">
                      {isSubmitting && <Loader2Icon className="h-4 w-4 animate-spin" color="hsl(0 0% 98%)" />}
                      <Text>Login</Text>
                    </View>
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
