import { Fragment } from "react";
import { Dimensions, SafeAreaView, View } from "react-native";

import { useQuery } from "@tanstack/react-query";
import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import { useLocalSearchParams, useRouter } from "expo-router";
import { ClapperboardIcon, PlayIcon, TvIcon } from "lucide-react-native";

import { useTRPC } from "~/lib/trpc";
import { cn } from "~/lib/utils";

import { Button } from "~/components/ui/button";
import { Text } from "~/components/ui/text";
import * as React from "react";

export default function Media() {
  const { mediaId } = useLocalSearchParams();
  const router = useRouter();
  const trpc = useTRPC();

  const { width, height } = Dimensions.get("window");

  const query = useQuery(trpc.media._media_id.get.queryOptions({ params: { mediaId: mediaId as string } }));

  return (
    <SafeAreaView className="flex min-h-screen w-full flex-col bg-background">
      <View className="relative h-[50vh] w-screen">
        <View className="ml-[98px] relative h-[50vh]">
          <Image
            className="object-cover"
            source={`http://192.168.86.123:8080/media/${query.data?.id}/fanart`}
            style={{ height: height / 2, width: width - 98 }}
          />
          <LinearGradient
            className="absolute inset-0"
            colors={["hsl(240 10% 3.9%)", "transparent"]}
            start={{ x: 0, y: 0.5 }}
            end={{ x: 1, y: 0.5 }}
          />
          <LinearGradient className="absolute inset-0" colors={["transparent", "hsl(240 10% 3.9%)"]} />
        </View>

        <View className="absolute w-full gap-8 pl-8 pr-12 pt-40">
          <Image className="h-[99.2px] w-[256px]" source={require("../../../../assets/images/clearlogo.png")} />

          {query.isSuccess && (
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
                {query.data!.type === "tvshow" && (
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
                  {query.data!.attributes.year || query.data!.attributes.premiered.split("-")[0]} •{" "}
                  {query.data!.attributes.studio}
                </Text>
                <Text className="text-lg">{query.data!.attributes.plot}</Text>
              </View>
            </View>
          )}
        </View>
      </View>
    </SafeAreaView>

  );
}
