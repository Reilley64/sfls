import "~/global.css";

import { Fragment } from "react";
import { View } from "react-native";

import { Slot } from "expo-router";

import { FilmIcon, HomeIcon, SettingsIcon, TvIcon } from "lucide-react-native";

import { cn } from "~/lib/utils";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";

export default function MainLayout() {
  return (
    <Fragment>
      <Slot />

      <View className="absolute z-10 flex h-screen shrink-0 grow-0 flex-col justify-center gap-4 bg-card/90 p-4">
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "flex h-auto w-20 flex-col gap-2 rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => console.log("home")}
        >
          {({ focused }) => (
            <Fragment>
              <HomeIcon className="!size-7" color="hsl(0 0% 98%)" />
              <Text className={cn("text-xs font-medium", focused && "text-accent-foreground")}>Home</Text>
            </Fragment>
          )}
        </Button>
        <Button
          variant="ghost"
          size="icon"
          className={cn(
            "flex h-auto w-20 flex-col gap-2 rounded-xl py-2",
            // activeSection === "home" ? "bg-purple-900/50 text-purple-400" : "text-gray-400 hover:text-white",
          )}
          onPress={() => console.log("movies")}
        >
          {({ focused }) => (
            <Fragment>
              <FilmIcon className="!size-7" color="hsl(0 0% 98%)" />
              <Text className={cn("text-xs font-medium", focused && "text-accent-foreground")}>Movies</Text>
            </Fragment>
          )}
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
          {({ focused }) => (
            <Fragment>
              <TvIcon className="!size-7" color="hsl(0 0% 98%)" />
              <Text className={cn("text-xs font-medium", focused && "text-accent-foreground")}>Shows</Text>
            </Fragment>
          )}
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
          {({ focused }) => (
            <Fragment>
              <SettingsIcon className="!size-7" color="hsl(0 0% 98%)" />
              <Text className={cn("text-xs font-medium", focused && "text-accent-foreground")}>Settings</Text>
            </Fragment>
          )}
        </Button>
      </View>
    </Fragment>
  );
}
