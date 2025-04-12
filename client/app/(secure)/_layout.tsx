import "~/global.css";

import { TextInput, View } from "react-native";
import { KeyboardAwareScrollView } from "react-native-keyboard-controller";

import { useForm } from "@tanstack/react-form";
import { useMutation } from "@tanstack/react-query";
import { Slot } from "expo-router";
import { GalleryVerticalEnd } from "lucide-react-native";
import { cssInterop } from "nativewind";

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
        auth.setBearerToken(data.token);
      },
    }),
  );

  const form = useForm({
    defaultValues: {
      email: "",
      password: "",
    },
    onSubmit: ({ value }) => {
      mutation.mutate({ body: value });
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
                      className="flex h-9 w-full animate-none rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground transition-none file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus:border-white disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
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
                      className="flex h-9 w-full animate-none rounded-md border border-input bg-transparent px-3 py-1 text-base text-foreground transition-none file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus:border-white disabled:cursor-not-allowed disabled:opacity-50 md:text-sm"
                      hasTVPreferredFocus
                      onBlur={field.handleBlur}
                      onChangeText={(value) => field.handleChange(value)}
                      textContentType="password"
                      value={field.state.value}
                    />
                  </View>
                )}
              />

              <Button onPress={form.handleSubmit}>
                <Text className="text-xs font-medium">Login</Text>
              </Button>
            </View>
          </View>
        </View>
      </KeyboardAwareScrollView>
    );
  }

  return <Slot />;
}
