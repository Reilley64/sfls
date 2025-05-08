import { Fragment } from "react";
import * as React from "react";
import { Dimensions, View } from "react-native";

import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import { useLocalSearchParams, useRouter } from "expo-router";

import { useQuery } from "@tanstack/react-query";
import { ClapperboardIcon, PlayIcon, TvIcon } from "lucide-react-native";
import Animated from "react-native-reanimated";

import { useAuthStore } from "~/stores/auth";

import { useTRPC } from "~/lib/trpc";
import { cn } from "~/lib/utils";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";

export default function Media() {
  const auth = useAuthStore();
  const { mediaId } = useLocalSearchParams();
  const router = useRouter();
  const trpc = useTRPC();

  const { width, height } = Dimensions.get("window");

  const query = useQuery(
    trpc.media._media_id.get.queryOptions({
      params: { mediaId: mediaId as string },
      headers: { Authorization: `Bearer ${auth.bearerToken}` },
    }),
  );

  return (
    <View className="flex min-h-screen w-full flex-col bg-background">
      <Animated.View
        className="relative ml-[98px]"
        sharedTransitionTag={`backdrop-${mediaId}`}
        style={{ height: height / 2, width: width - 98 }}
      >
        <Image
          cachePolicy="memory"
          className="h-full w-full object-cover"
          source={`http://192.168.86.215:10000/media/${mediaId}/images/background`}
        />
        <LinearGradient
          className="absolute inset-0"
          colors={["hsl(240 10% 3.9%)", "transparent"]}
          start={{ x: 0, y: 0.5 }}
          end={{ x: 1, y: 0.5 }}
        />
        <LinearGradient className="absolute inset-0" colors={["transparent", "hsl(240 10% 3.9%)"]} />
        <Image
          cachePolicy="memory"
          className="absolute bottom-0"
          source={`http://192.168.86.215:10000/media/${mediaId}/images/logo`}
          style={{ height: 99.2, width: 256 }}
        />
      </Animated.View>

      {query.data && (
        <View className="w-full gap-8 p-8 pr-12">
          <View className="flex w-full flex-row gap-8">
            <View className="flex shrink-0 grow-0 basis-1/3 flex-col gap-4">
              <Button
                className="focus:bg-white"
                hasTVPreferredFocus
                onPress={() => {
                  router.navigate({
                    pathname: "/media/[mediaId]/stream",
                    params: { mediaId: mediaId as string },
                  });
                }}
                variant="ghost"
              >
                {({ focused }) => (
                  <Fragment>
                    <PlayIcon color={focused ? "black" : "hsl(0 0% 98%)"} height={16} width={16} />
                    <Text className={cn("font-bold", focused && "text-black")}>Play</Text>
                  </Fragment>
                )}
              </Button>
              <Button className="focus:bg-white" variant="ghost">
                {({ focused }) => (
                  <Fragment>
                    <ClapperboardIcon color={focused ? "black" : "hsl(0 0% 98%)"} height={16} width={16} />
                    <Text className={cn("font-bold", focused && "text-black")}>Trailer</Text>
                  </Fragment>
                )}
              </Button>
              {query.data.type === "tvshow" && (
                <Button className="focus:bg-white" variant="ghost">
                  {({ focused }) => (
                    <Fragment>
                      <TvIcon color={focused ? "black" : "hsl(0 0% 98%)"} height={16} width={16} />
                      <Text className={cn("font-bold", focused && "text-black")}>Episodes</Text>
                    </Fragment>
                  )}
                </Button>
              )}
            </View>

            <View className="flex shrink-0 grow-0 basis-2/3 flex-col gap-2">
              <Text>
                {query.data.attributes.year || query.data.attributes.premiered.split("-")[0]} •{" "}
                {query.data.attributes.studio}
              </Text>
              <Text className="text-lg">{query.data.attributes.plot}</Text>
            </View>
          </View>
        </View>
      )}
    </View>
  );
}
