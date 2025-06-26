import * as React from "react";
import { ComponentRef, useRef, useState } from "react";
import { Dimensions, FlatList, Pressable, SafeAreaView, SectionList, View } from "react-native";

import { Image } from "expo-image";
import { LinearGradient } from "expo-linear-gradient";
import { useRouter } from "expo-router";

import { useQueryClient, useSuspenseQueries } from "@tanstack/react-query";
import { cssInterop } from "nativewind";
import Animated from "react-native-reanimated";

import { useAuthStore } from "~/stores/auth";

import { RouterOutput, useTRPC } from "~/lib/trpc";
import { cn } from "~/lib/utils";

import { Text } from "~/components/ui/text";

cssInterop(Image, {
  className: {
    target: "style",
  },
});

export default function Home() {
  const auth = useAuthStore();
  const queryClient = useQueryClient();
  const router = useRouter();
  const trpc = useTRPC();

  const { width, height } = Dimensions.get("window");

  const [continueQuery, movieQuery, showQuery] = useSuspenseQueries({
    queries: [
      trpc.media.continue.get.queryOptions({ headers: { Authorization: `Bearer ${auth.bearerToken}` } }),
      trpc.media.get.queryOptions({
        query: { types: ["movie"], orderBy: "Random" },
        headers: { Authorization: `Bearer ${auth.bearerToken}` },
      }),
      trpc.media.get.queryOptions({
        query: { types: ["tvshow"], orderBy: "Random" },
        headers: { Authorization: `Bearer ${auth.bearerToken}` },
      }),
    ],
  });

  const sections = (() => {
    const result = [];

    if (!continueQuery.isError) {
      const continueList = continueQuery.data!;
      if (continueList.length > 0) {
        result.push({ title: "Continue Watching", index: result.length, data: [continueList] });
      }
    }

    if (!movieQuery.isError) {
      const movieList = movieQuery.data!;
      if (movieList.length > 0) {
        result.push({ title: "Movies", index: result.length, data: [movieList.slice(0, 5)] });
      }
    }

    if (!showQuery.isError) {
      const showList = showQuery.data!;
      if (showList.length > 0) {
        result.push({ title: "Shows", index: result.length, data: [showList.slice(0, 5)] });
      }
    }

    return result;
  })();

  const [focusedMedia, setFocusedMedia] = useState<RouterOutput["media"]["_media_id"]["get"]>(
    sections[0]?.data[0]?.[0],
  );

  const sectionListRef = useRef<ComponentRef<
    typeof SectionList<Array<RouterOutput["media"]["_media_id"]["get"]>>
  > | null>(null);
  const flatListRefs = useRef<Array<ComponentRef<typeof FlatList<RouterOutput["media"]["_media_id"]["get"]>> | null>>(
    [],
  );

  function handleListItemFocus(x: number) {
    return (y: number) => {
      sectionListRef.current?.scrollToLocation({ sectionIndex: x, itemIndex: 0, animated: true });
      flatListRefs.current[x]?.scrollToIndex({ index: y, animated: true, viewOffset: 98 });
      setFocusedMedia(sections[x]!.data[0][y]);
    };
  }

  return (
    <View className="max-w-screen flex min-h-screen w-full flex-col gap-6 bg-background">
      {focusedMedia && (
        <Animated.View
          className="relative ml-[98px]"
          sharedTransitionTag={`backdrop-${focusedMedia.id}`}
          style={{ height: height / 2 - 24, width: width - 98 }}
        >
          <Image
            cachePolicy="memory"
            className="h-full w-full object-cover"
            source={`http://192.168.86.215:10000/media/${focusedMedia.id}/images/background`}
          />
          <LinearGradient
            className="absolute inset-0"
            colors={["black", "transparent"]}
            start={{ x: 0, y: 0.5 }}
            end={{ x: 1, y: 0.5 }}
          />
          <LinearGradient className="absolute inset-0" colors={["transparent", "black"]} />
          <Image
            cachePolicy="memory"
            className="absolute bottom-0"
            source={`http://192.168.86.215:10000/media/${focusedMedia.id}/images/logo`}
            style={{ height: 99.2, width: 256 }}
          />
        </Animated.View>
      )}

      <View className="relative w-full">
        <SectionList<Array<RouterOutput["media"]["_media_id"]["get"]>>
          ref={sectionListRef}
          contentContainerStyle={{ paddingBottom: 10000 }}
          sections={sections}
          scrollEnabled={false}
          renderItem={({ item: data, section: { index: x } }) => (
            <FlatList<RouterOutput["media"]["_media_id"]["get"]>
              ref={(el) => {
                flatListRefs.current[x] = el;
              }}
              contentContainerStyle={{ paddingRight: 10000 }}
              data={data}
              horizontal
              scrollEnabled={false}
              showsHorizontalScrollIndicator={false}
              renderItem={({ item: media, index: y }) => (
                <Pressable
                  className={cn("rounded-xl p-2 transition-transform", y === 0 && "ml-[98px]")}
                  onPress={() => {
                    queryClient.setQueryData(
                      trpc.media._media_id.get.queryKey({ params: { mediaId: media.id } }),
                      media,
                    );

                    router.navigate({
                      pathname: "/media/[mediaId]",
                      params: { mediaId: media.id },
                    });
                  }}
                  onFocus={() => handleListItemFocus(x)(y)}
                  hasTVPreferredFocus={x === 0 && y === 0}
                  isTVSelectable={true}
                >
                  <Image
                    className="rounded-xl"
                    key={media.id}
                    source={`http://192.168.86.215:10000/media/${media.id}/images/poster`}
                    style={{ width: 1000 / 7, height: 1426 / 7 }}
                  />
                </Pressable>
              )}
            />
          )}
          renderSectionHeader={({ section: { title } }) => (
            <Text className="ml-[98px] font-[Geist] text-xl font-semibold text-foreground">{title}</Text>
          )}
        />

        <View
          className="absolute left-[101px] top-[28px] rounded-2xl border-2 border-white"
          style={{ width: 1000 / 7 + 8, height: 1426 / 7 + 8 }}
        />
      </View>
    </View>
  );
}
