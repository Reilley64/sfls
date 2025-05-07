import { useEffect } from "react";
import { Dimensions, SafeAreaView } from "react-native";

import { DefaultError, useMutation } from "@tanstack/react-query";
import { useLocalSearchParams } from "expo-router";
import { VideoView, useVideoPlayer } from "expo-video";
import { cssInterop } from "nativewind";

import { useAuthStore } from "~/stores/auth";

import { RouterOutput, useTRPC } from "~/lib/trpc";

cssInterop(VideoView, {
  className: {
    target: "style",
  },
});

export default function Stream() {
  const { bearerToken } = useAuthStore();
  const { mediaId } = useLocalSearchParams();
  const trpc = useTRPC();

  const { width, height } = Dimensions.get("window");

  const player = useVideoPlayer(
    {
      uri: `http://192.168.86.123:10000/media/${mediaId}/stream`,
      headers: { Authorization: `Bearer ${bearerToken}` },
    },
    (player) => {
      player.play();
    },
  );

  const heartbeatMutation = useMutation(trpc.media._media_id.stream.heartbeat.post.mutationOptions());

  useEffect(() => {
    const interval = setInterval(() => {
      heartbeatMutation.mutate({ params: { mediaId: mediaId as string }, body: { position: player.currentTime } });
    }, 900000);

    return () => clearInterval(interval);
  }, [heartbeatMutation]);

  return (
    <SafeAreaView className="bg-backgroud">
      <VideoView className="bg-black" player={player} style={{ width, height }} />
    </SafeAreaView>
  );
}
