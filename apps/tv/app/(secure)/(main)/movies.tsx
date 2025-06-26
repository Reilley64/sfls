import * as React from "react";
import { FlatList, Pressable, View, VirtualizedList, useWindowDimensions } from "react-native";

import { Image } from "expo-image";
import { useRouter } from "expo-router";

import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useAuthStore } from "~/stores/auth";

import { RouterOutput, useTRPC } from "~/lib/trpc";
import { cn } from "~/lib/utils";
import { useMemo } from "react";

export default function Movies() {
  const auth = useAuthStore();
  const queryClient = useQueryClient();
  const router = useRouter();
  const trpc = useTRPC();
  const { width, height } = useWindowDimensions();

  const movieQuery = useQuery(
    trpc.media.get.queryOptions({
      query: { types: ["movie"], orderBy: "Title" },
      headers: { Authorization: `Bearer ${auth.bearerToken}` },
    }, {
      initialData: [] as RouterOutput["media"]["get"],
    }),
  );

  const splitData = useMemo(() => {
    let data = [];
    for (let x = 0; x < movieQuery.data.length; x++) {
      let items = []
      for (let y = 0; y < 3; y++) {
        const item = movieQuery.data[x * 3 + y];
        item && items.push(item);
      }
      data.push(items);
    }
    return data;
  }, [JSON.stringify(movieQuery.data)]);

  return (
    <View className="min-w-screen flex min-h-screen flex-col bg-background">
      <FlatList
        contentContainerStyle={{ paddingLeft: 98 }}
        data={splitData}
        horizontal
        renderItem={({ item, index: x }) => (
          <VirtualizedList
            data={item}
            getItem={(data, index) => data[index]}
            getItemCount={(data) => data.length}
            keyExtractor={(item) => item.id}
            showsHorizontalScrollIndicator={false}
            renderItem={({ item: media, index: y }) => (
              <Pressable
                key={media.id}
                className={cn("rounded-xl p-[1px] transition-transform border-2 border-transparent focus:border-white")}
                onPress={() => {
                  queryClient.setQueryData(trpc.media._media_id.get.queryKey({ params: { mediaId: media.id } }), media);

                  router.navigate({
                    pathname: "/media/[mediaId]",
                    params: { mediaId: media.id },
                  });
                }}
                hasTVPreferredFocus={x === 0 && y === 0}
                isTVSelectable={true}
              >
                <Image
                  className="rounded-xl"
                  key={media.id}
                  source={`http://192.168.86.215:10000/media/${media.id}/images/poster`}
                  style={{ aspectRatio: "1000 / 1426", height: (height - 20) / 3 }}
                />
              </Pressable>
            )}
            horizontal
          />
        )}
      />
    </View>
  );
}
