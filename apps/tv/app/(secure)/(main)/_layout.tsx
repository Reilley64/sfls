import "~/global.css";

import { Fragment } from "react";
import { View } from "react-native";

import { Slot, useRouter } from "expo-router";

import { FilmIcon, HomeIcon, SettingsIcon, TvIcon } from "lucide-react-native";

import { cn } from "~/lib/utils";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";

export default function MainLayout() {
  const router = useRouter();

  return (
    <Fragment>
      <Slot />

      <View className="absolute z-10 flex h-screen shrink-0 grow-0 flex-col justify-center gap-4 bg-background/90 p-4">
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "flex h-auto w-20 flex-col gap-2 rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => console.log("home")}
        >
          <HomeIcon color="hsl(0 0% 98%)" />
          <Text>Home</Text>
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "flex h-auto w-20 flex-col gap-2 rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => router.navigate("/movies")}
        >
          <FilmIcon color="hsl(0 0% 98%)" />
          <Text>Movies</Text>
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "group flex h-auto w-20 flex-col gap-2 rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => console.log("shows")}
        >
          <TvIcon color="hsl(0 0% 98%)" />
          <Text>Shows</Text>
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "flex h-auto w-20 flex-col rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => console.log("settings")}
        >
          <SettingsIcon color="hsl(0 0% 98%)" />
          <Text>Settings</Text>
        </Button>
      </View>
    </Fragment>
  );
}
